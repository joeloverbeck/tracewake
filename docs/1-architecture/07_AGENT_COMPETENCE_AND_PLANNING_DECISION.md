# Agent Competence and Planning Decision

## Core decision

Tracewake v1 agents are symbolic, planner-driven, inspectable agents. They are not LLM brains.

The chosen architecture is:

```text
body and needs
  ↓
identity, traits, roles, relationships
  ↓
beliefs, memories, expectations
  ↓
values, motives, life goals
  ↓
current concerns and candidate goals
  ↓
durable projects and intentions
  ↓
HTN method selection
  ↓
bounded local planning
  ↓
primitive actions
  ↓
observation, surprise, interruption, learning
```

This stack should produce ordinary human competence: eating, sleeping, working, asking, refusing, lying, recruiting, searching, hiding, reporting, investigating poorly, avoiding danger, following duties, breaking duties, and adapting to partial information.

## Why not LLM agents

LLM-driven agents can appear socially rich, but Tracewake requires replayability, causal inspection, strict actor knowledge, bounded resources, deterministic tests, and institutional validity. LLMs are stochastic, opaque, bias-prone, hallucination-prone, and difficult to validate as authoritative decision systems.

LLMs may later improve language surfaces. They may not decide what agents do in v1.

## BDI-style separation

Use BDI as architecture doctrine:

### Beliefs

Subjective propositions with source, confidence, channel, time, and contradiction links.

```yaml
Belief:
  holder: actor_tomas
  proposition: ItemLocatedInContainer(coin_stack_01, strongbox_tomas)
  truth_value: believed_true
  confidence: 0.86
  source:
    kind: prior_direct_observation
    source_id: evt_checked_strongbox_yesterday
  acquired_at: 142-08-11T20:10
```

### Desires, values, goals, and pressures

Tracewake does not need a single “desire” bucket. It needs multiple kinds of motivation:

- immediate needs: hunger, fatigue, safety, warmth, pain;
- stable values: family, property, law, faith, status, freedom, comfort, curiosity;
- contextual motives: recover property, avoid shame, protect kin, escape debt, conceal wrongdoing, obey authority;
- role obligations: patrol route, open shop, report serious threat, feed child;
- life goals: save money, marry, leave town, become guild member, hide a crime, repair reputation.

### Intentions

Intentions are committed courses of action. They persist until completed, impossible, superseded, contradicted, too costly, socially overridden, institutionally ordered away, or abandoned.

Avoid utility jitter. A hungry agent should not abandon a report every tick because another score twitched.

## Agent state

```yaml
AgentMind:
  actor: actor_tomas
  traits:
    risk_tolerance: low
    conscientiousness: high
    suspicion: medium
  needs:
    hunger: 22
    fatigue: 66
    safety: 80
  values:
    family: 0.91
    property: 0.83
    lawfulness: 0.71
  life_goals:
    - keep_mill_solvent
    - protect_household
  roles:
    - miller
    - household_head
  relationships:
    actor_elena:
      kinship: spouse
      trust: 0.88
      affection: 0.81
  active_projects:
    - project_pay_tax_due
  current_intention: intention_go_to_work_991
```

## Needs

Start with hunger, fatigue, and safety.

Needs create pressure and candidate goals, not hard compulsion.

```yaml
NeedPressure:
  hunger:
    ordinary_methods: [eat_at_home, buy_food, eat_at_tavern]
    desperate_methods: [beg, steal_food]
  fatigue:
    ordinary_methods: [sleep_at_home, nap]
    desperate_methods: [sleep_in_public, collapse]
  safety:
    ordinary_methods: [avoid_threat, seek_guard, stay_home]
    desperate_methods: [flee_region, arm_self]
```

This `NeedPressure` block is the canonical needs model; `10_ORDINARY_LIFE_ECONOMY_AND_SETTLEMENTS.md` re-presents the same needs as ordinary-life action menus, not as a competing model.

## Expectations and surprise

Discovery often requires expectation.

```text
expected: coins in strongbox
observed: strongbox empty
contradiction: high
candidate goals:
  - search room
  - ask household
  - report theft
  - conceal loss
  - accuse likely suspect
  - update belief about storage
```

Surprise is mechanical. It creates candidate goals, emotional shifts, belief updates, and trace inspection opportunities.

## Projects

A project is a persistent multi-step concern.

```yaml
Project:
  owner: actor_tomas
  goal: MaintainBusinessSolvency
  horizon: months
  importance: high
  subgoals:
    - collect_debts
    - buy_grain
    - pay_tax_due
```

Projects create narrative continuity without authored arcs.

## HTN methods

HTN methods are used for high-level routines and procedures.

Examples:

- Workday;
- EatMeal;
- SleepNight;
- RecoverMissingProperty;
- HideStolenItem;
- ReportIncident;
- OpenCase;
- PostNotice;
- PatrolRoute;
- RecruitCompanion;
- TravelToSite;
- InspectSuspiciousPlace.

Example:

```text
RecoverMissingProperty(item)
  Method: PrivateSearch
    Preconditions:
      - actor knows last known place
      - actor can access or is willing to violate access norm
    Steps:
      - go to last known location
      - inspect container
      - inspect traces
      - search plausible nearby storage
    Failures:
      - access denied
      - too dark
      - interrupted
      - nothing found

  Method: AskHousehold
    Preconditions:
      - household member reachable
      - relationship or authority sufficient
    Steps:
      - locate household member
      - ask about item
      - interpret answer
    Failures:
      - refusal
      - lie
      - uncertain memory
      - social cost too high

  Method: ReportToAuthority
    Preconditions:
      - authority reachable
      - actor believes authority may help
    Steps:
      - travel to office
      - wait for clerk if needed
      - make report
      - receive response
    Failures:
      - office closed
      - report not credible
      - clerk absent
      - fee required
```

Methods must not guarantee outcomes. They select possible behavior under state.

## Bounded local planning

Use bounded GOAP-style planning for concrete means:

- reach target room;
- open or unlock door;
- find food;
- fetch tool;
- inspect container;
- carry item;
- speak to target;
- leave private room unnoticed;
- travel to office;
- flee threat.

The local planner should have explicit budgets:

- max nodes;
- max depth;
- max simulated time;
- LOD-dependent depth;
- permitted action set;
- fallback action;
- failure event when budget exhausted.

Do not make every actor solve broad life strategy through GOAP every tick.

## Event-driven replanning

Agents reconsider when:

- need threshold crosses;
- routine phase begins;
- plan step completes or fails;
- observation contradicts important belief;
- report/order/message arrives;
- danger is perceived;
- resource becomes unavailable;
- social obligation changes;
- institution issues command;
- travel segment completes;
- LOD tier changes;
- debug possession attaches to actor if necessary for controls.

## Social intelligence

Social actions are mechanical, not merely prose.

Required speech/social acts include:

- ask;
- answer;
- report;
- tell;
- gossip;
- lie;
- deny;
- confess;
- accuse;
- threaten;
- bargain;
- promise;
- recruit;
- refuse;
- instruct;
- command;
- testify;
- withhold;
- apologize.

Choices depend on relationship, trust, fear, status, role, motive, risk, norms, setting, and current beliefs.

The canonical speech-act vocabulary lives in `08_INFORMATION_ECOLOGY_AND_SPEECH_ACTS.md`; the list above is a working subset scoped to agent cognition.

## Emotion

Emotion modulates planning. It does not author outcomes.

- Fear increases avoidance and seeking safety.
- Anger increases confrontation and risk tolerance.
- Shame increases concealment.
- Grief interrupts routine.
- Affection increases aid.
- Suspicion increases inspection.
- Relief reduces urgency.
- Guilt increases confession or defensive lies depending on traits.

Emotion must have causes and should be inspectable.

## Agent LOD

Not every agent runs full detail always.

```text
Tier A — embodied/high-salience agent:
  full beliefs, needs, projects, speech, planning, perception

Tier B — local important agent:
  detailed intentions, simplified retrieval, active routines

Tier C — routine citizen:
  routine templates, salient beliefs, limited planning

Tier D — background population:
  summary events, promotable ancestry
```

Lower LOD is not nonexistence. Promotion must restore enough detail for causality and current relevance.

These agent-detail tiers (A–D) are orthogonal to the simulation-scope tiers (0–3) in `17_SIMULATION_LOD_TIME_AND_PERFORMANCE.md`: a Tier-A embodied agent runs inside Tier-0 simulation, while Tier-D background population is carried by the Tier-2/Tier-3 scope.

## Minimal vertical-slice cognition

The first serious slice requires:

- hunger/fatigue/safety;
- belief store with confidence/source;
- direct observation;
- overheard sound;
- speech report;
- expectation contradiction;
- current intention;
- one project for major actors;
- relationships/trust;
- routine HTN for sleep/eat/work/social;
- theft/report/notice methods;
- bounded local planning for movement and object use;
- lying and refusal;
- interruption;
- no-player autonomy;
- planner debug output;
- deterministic tests.

## Anti-patterns

- LLM decides agent action.
- Agent knows truth because it is convenient.
- Schedule teleports NPCs.
- Need meter fully controls behavior.
- Every actor replans every tick.
- GOAP is used as whole personality.
- HTN method is a hidden plot script.
- NPC exists only to react to player.
- Player body gets exclusive actions.
- Debug possession erases prior intentions.
