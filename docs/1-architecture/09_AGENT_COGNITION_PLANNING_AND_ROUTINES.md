# Agent Cognition, Planning, and Routines

## Status

This document defines the symbolic, inspectable agent cognition architecture for Tracewake.

V1 agents are not LLM brains. They are ordinary agents with bodies, needs, beliefs, memories, relationships, values, roles, routines, projects, intentions, and bounded planning.

## Core architecture

Tracewake uses a BDI-style separation with routine/procedure decomposition and bounded local planning.

```text
body and needs
 -> identity, traits, values, roles, relationships
 -> beliefs, memories, expectations
 -> pressures, motives, projects, candidate goals
 -> durable intention selection
 -> HTN method/procedure selection
 -> bounded local planning for concrete steps
 -> action proposals through shared pipeline
 -> observation, surprise, failure, learning, replanning
```

This architecture should produce ordinary competence: eating, sleeping, working, storing property, asking, reporting, lying, refusing, gossiping, searching, hiding, traveling, obeying, violating, and adapting to partial information.

## Why not LLM agents

Tracewake requires deterministic replay, actor-knowledge filtering, causality inspection, planner traces, validation, no-human simulation, and institutional fallibility. LLM-driven agent brains are stochastic, opaque, difficult to validate, prone to unsupported facts, and poor as authoritative planners.

LLMs may later render or parse language behind validation. They do not decide agent actions.

## Agent mind shape

```yaml
AgentMind:
  actor: actor_mara
  traits:
    risk_tolerance: medium
    conscientiousness: low
    suspicion: medium
    shame_sensitivity: high
  needs:
    hunger: 41
    fatigue: 38
    safety: 64
  values:
    household: 0.62
    property_respect: 0.31
    reputation: 0.73
    survival: 0.88
  roles:
    - household_member
    - part_time_worker
  relationships:
    actor_tomas:
      trust: 0.22
      resentment: 0.55
      fear: 0.10
  active_projects:
    - repay_debt_before_market_day
  current_intention:
    id: intention_find_money_quietly
    durability: high
  planner_budget_profile: local_ordinary_agent
```

This is a conceptual data shape, not final implementation code.

## Belief store interface

Agent cognition reads from actor-specific beliefs, not ground truth.

Required belief queries:

- what does the actor believe about current location?
- what does the actor believe about owned/expected items?
- what routes/places does the actor know?
- what institutions or roles does the actor believe can help?
- who does the actor trust or fear?
- what rumors or records has the actor heard/read?
- what contradictions are salient?
- what beliefs are private, shameful, speakable, or admissible?

The belief store must return source and confidence enough for action proposals and debug explanations.

## Needs and pressures

Start with hunger, fatigue, and safety.

Needs create pressures and candidate goals. They do not hard-script behavior.

```yaml
NeedPressurePolicy:
  hunger:
    ordinary_methods: [eat_at_home, buy_food, eat_at_tavern]
    desperate_methods: [beg, steal_food]
    affects:
      - attention
      - irritability
      - willingness_to_delay
  fatigue:
    ordinary_methods: [sleep_at_home, nap]
    desperate_methods: [sleep_in_public, collapse]
    affects:
      - perception
      - planning_budget
      - work_quality
  safety:
    ordinary_methods: [avoid_threat, seek_guard, stay_home, travel_with_companion]
    desperate_methods: [flee_region, hide, arm_self]
    affects:
      - route_choice
      - risk_tolerance
      - social_disclosure
```

Needs should interact with routines and projects. A hungry clerk may still receive a report if duty, reputation, and office hours outweigh the hunger pressure.

## Values, traits, and relationships

Values and traits shape choices, not outcomes.

Examples:

- high lawfulness raises cost of theft and false testimony;
- high risk tolerance lowers avoidance;
- high family value increases household aid;
- low trust reduces belief uptake;
- fear increases avoidance and seeking safety;
- resentment increases hostile interpretation;
- shame increases concealment or confession depending on traits.

Every major emotional or relationship shift should have a causal source.

## Projects and durable intentions

A project is a persistent concern with a horizon.

```yaml
Project:
  id: project_repay_debt
  owner: actor_mara
  goal: RepayDebt(actor_mara, actor_iva)
  horizon: days
  importance: high
  source:
    kind: debt_record_or_memory
    id: debt_mara_iva_01
  candidate_methods:
    - work_extra_shift
    - ask_household
    - borrow_from_friend
    - steal_food_or_coin
    - flee_or_hide
```

Intentions are commitments. They persist until completed, impossible, superseded, contradicted, too costly, socially overridden, institutionally ordered away, or abandoned by explicit event.

Avoid utility jitter. Agents should not switch every tick because a score moved slightly.

## Candidate goal generation

Candidate goals arise from:

- need thresholds;
- routine phases;
- role obligations;
- household obligations;
- projects;
- observed opportunities;
- expectation contradictions;
- reports/messages/orders;
- perceived danger;
- social requests;
- institutional summons;
- memory salience;
- LOD promotion.

Candidate goals include source, urgency, expected cost, risk, and reason.

```yaml
CandidateGoal:
  kind: RecoverMissingProperty
  owner: actor_tomas
  source: expectation_contradiction_strongbox_empty
  urgency: high
  emotional_tone: alarm
  possible_methods: [private_search, ask_household, report_to_authority]
```

## Intention selection

Intention selection considers:

- current durable intention;
- need pressure;
- project importance;
- role obligation;
- household obligation;
- social cost;
- physical feasibility;
- believed opportunity;
- danger;
- time windows;
- planner budget;
- LOD tier.

Utility scoring may be used as a bounded heuristic. It must not be the whole mind and must not create action without causal motive.

## HTN methods

HTN methods handle high-level routines and procedures.

Examples:

- EatMeal;
- SleepNight;
- Workday;
- StoreValuable;
- SearchForMissingItem;
- AskHouseholdAboutItem;
- ReportIncident;
- ReceiveReport;
- OpenIncidentRecord;
- QuestionWitness;
- PostNotice;
- HideStolenItem;
- TravelToPlace;
- VisitMarket;
- CloseShop.

Example method:

```yaml
HTNMethod:
  task: RecoverMissingProperty(item)
  method: AskHousehold
  applicability:
    - actor_believes_item_missing
    - actor_knows_household_members
    - social_cost_acceptable
  steps:
    - locate_household_member
    - ask_about_item
    - interpret_answer
    - update_suspicion_or_continue_search
  failure_modes:
    - household_member_unreachable
    - refusal
    - lie
    - uncertain_memory
    - social_cost_too_high
```

HTN methods select possible behavior. They do not guarantee outcomes.

## Bounded local planning

Local planning solves concrete action sequences:

- reach a room;
- open a door;
- obtain food;
- use bed;
- fetch key/tool;
- inspect container;
- search room;
- carry item;
- speak to target;
- avoid witness;
- travel to office;
- wait for office hours.

Planner budgets must be explicit:

```yaml
PlannerBudget:
  max_nodes: 200
  max_depth: 12
  max_simulated_duration: 2h
  max_world_queries: bounded
  allowed_action_categories: [movement, object_use, speech, wait]
  fallback_actions: [wait, reconsider, ask_for_help, abandon_intention]
  on_exhaustion_event: PlanningFailed
```

A local planner proposes actions through the shared pipeline. It does not mutate world state.

## Routines as defeasible intentions

Bad:

```text
08:00 blacksmith goes to forge.
```

Good:

```text
At morning work phase, blacksmith considers Workday because she believes today is a workday,
has orders, needs income, can reach the forge, has tools/fuel, is not too exhausted,
and no stronger interruption dominates.
```

Routines should fail or change when:

- food is missing;
- fatigue is too high;
- door is locked;
- workplace is unsafe;
- child is sick;
- clerk summons actor;
- route is believed dangerous;
- theft discovery interrupts;
- tool is unavailable;
- social conflict erupts.

Schedules create expectations. They are not teleports.

## Event-driven replanning

Agents reconsider on:

- action step completion;
- action failure or rejection;
- important observation;
- expectation contradiction;
- speech/report/order;
- need threshold;
- routine phase;
- danger;
- resource loss;
- social request;
- institutional procedure;
- LOD promotion/demotion;
- possession attachment only when needed to expose controls, not to reset mind.

Replanning should leave inspectable traces when it changes important intentions.

## Planner trace/debug output

Debug output must answer:

- what did the actor believe?
- what needs/pressures were active?
- what candidate goals were considered?
- what intention was chosen and why?
- what HTN method was selected?
- what local plan was attempted?
- what precondition failed?
- what action proposal was submitted?
- what event or observation caused replanning?
- what hidden truth, if any, differs from the actor's belief?

Example:

```yaml
PlannerTrace:
  actor: actor_tomas
  trigger: expectation_contradiction_strongbox_empty
  candidate_goals:
    - private_search: score high
    - report_to_authority: score medium
    - accuse_mara: score low_due_insufficient_basis
  selected_intention: private_search
  selected_method: SearchRoomAndContainer
  rejected_actions:
    - accuse_mara: actor_lacks_specific_basis
```

Embodied why-not explanations are actor-filtered. Debug can show all validation details.

## Agent LOD

Per-agent detail tiers are separate from simulation-scope tiers.

```text
Agent Detail A: possessed/high-salience
  full needs, beliefs, projects, planning, speech, memory, perception

Agent Detail B: local important
  detailed intentions, simplified retrieval, active routines, salient beliefs

Agent Detail C: routine citizen
  routine templates, salient beliefs, limited planning, summary memory

Agent Detail D: background/promotable
  summary events, compressed projects, minimal needs, promotable ancestry
```

Lower detail is not nonexistence. Promotion must restore enough ancestry, beliefs, relationships, and current obligations for causal honesty.

## First-slice cognition

Minimum serious first slice:

- hunger/fatigue/safety;
- direct observation and sound observation;
- belief store with source/confidence;
- expectation contradiction;
- simple memory salience;
- current intention;
- at least one durable project for major actors;
- relationships/trust;
- routine HTN for sleep/eat/work/social;
- property storage and missing-item recovery methods;
- report/record/notice methods;
- bounded local planning for movement/object use/speech;
- lying/refusal;
- interruption;
- no-human autonomy;
- planner debug output;
- deterministic tests.

## Anti-patterns

- LLM chooses agent action.
- GOAP is the whole mind.
- Utility scores produce causeless behavior.
- Schedule teleports actors.
- NPC exists only to react to player.
- Player possession erases prior intentions.
- Agent knows ground truth because it is convenient.
- Planner mutates state directly.
- HTN method is a hidden plot script.
- Every actor replans every tick.
- Need meters are cosmetic.
