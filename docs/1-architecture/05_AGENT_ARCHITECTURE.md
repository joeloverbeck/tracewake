# Agent Architecture

## Goal

Tracewake agents should feel alive because they persistently pursue humanly understandable goals from partial, fallible, socially situated beliefs.

Target **ordinary human competence**: work, gossip, misremember, plan, recruit, hide, investigate, accuse, flee, lie, protect loved ones, pursue long-term projects, and adapt when reality contradicts them.

## Competence stack

```text
Body and needs
  ↓
Identity, traits, roles, relationships
  ↓
Beliefs, memories, expectations
  ↓
Values, motives, life goals
  ↓
Current concerns and candidate goals
  ↓
Intentions and projects
  ↓
HTN method selection
  ↓
Bounded local planning
  ↓
Primitive actions
  ↓
Observation, surprise, interruption, learning
```

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
  current_intention: intention_go_to_work_991
  active_projects:
    - project_pay_tax_due
```

## Beliefs

A belief is a subjective proposition with confidence and provenance.

```yaml
Belief:
  holder: actor_tomas
  proposition: ItemLocatedInContainer(coin_stack_01, strongbox_tomas)
  truth_value: believed_true
  confidence: 0.86
  source:
    kind: direct_observation
    source_id: evt_checked_strongbox_yesterday
  acquired_at: 142-08-11T20:10
```

Beliefs support direct observation, hearsay, rumor, records, inference, instruction, memory, lies received as testimony, and special channels defined by domain packs.

## Expectations and surprise

Discovery requires expectation.

```text
expected: coins in strongbox
observed: strongbox empty
contradiction: high
emotional response: alarm + anger
candidate goals:
  - search room
  - ask Elena
  - report theft
  - conceal loss
  - accuse likely suspect
```

Surprise is mechanical. It creates candidate goals and emotional shifts.

## Needs, values, motives, and life goals

Needs are immediate pressures: hunger, fatigue, pain, safety, warmth, shelter, social contact.

Values are stable priorities: family, wealth, law, faith, honor, revenge, status, comfort, freedom, curiosity.

Motives are contextual drives: recover property, avoid shame, obey authority, protect child, escape debt, conceal wrongdoing.

Life goals span days to years: save money, marry, become guild member, leave town, build house, avenge kin, retire, cure illness, hide a crime.

## Projects

A project is a persistent multi-step intention that can survive interruption.

```yaml
Project:
  owner: actor_tomas
  goal: MaintainBusinessSolvency
  horizon: months
  subgoals:
    - collect_debts
    - buy_grain
    - pay_tax_due
```

Projects make characters narratively compelling without authored arcs.

## Intentions

An intention persists until completed, impossible, superseded by emergency, contradicted by new belief, too costly, socially overridden, institutionally ordered away, or abandoned. Avoid utility jitter.

## HTN methods

Use HTN for high-level procedures:

```text
RecoverMissingProperty(item)
  Method: PrivateSearch
    - go to last known location
    - inspect container
    - inspect traces
    - ask household
  Method: ReportToAuthority
    - travel to office
    - wait for clerk
    - make report
  Method: ConfrontSuspect
    - locate suspect
    - accuse or question
    - escalate or withdraw
```

Method selection depends on beliefs, traits, status, trust, evidence, urgency, fear, relationships, and time.

## Local planning

Use bounded GOAP-style planning for concrete action sequences: reach location, open door, get tool, unlock container, speak to target, flee.

Do not make every actor solve broad life problems from scratch every tick.

## Social intelligence

Model social actions mechanically:

- ask;
- answer;
- refuse;
- lie;
- confess;
- threaten;
- bargain;
- promise;
- accuse;
- apologize;
- recruit;
- dismiss;
- flatter;
- intimidate;
- report;
- gossip;
- withhold.

Social choices depend on relationship, status, fear, trust, role, motive, and norms.

## Emotion

Emotion modulates planning. Fear increases avoidance. Anger increases confrontation. Shame increases concealment. Grief interrupts routine. Affection increases aid. Suspicion increases inspection. Emotion must have causes.

## Agent LOD

Not every agent runs full detail always.

- Tier A: embodied/high-salience agent — full beliefs, goals, speech, and planning.
- Tier B: local important agent — detailed intentions, simplified retrieval.
- Tier C: routine citizen — routine templates and salient beliefs.
- Tier D: background population — summary events, promotable when relevant.

Lower LOD is dormant detail, not disposable mannequins.

## Minimal vertical-slice cognition

Implement belief store, direct observation, speech report, expectation contradiction, hunger/fatigue/safety, current intention, one project per major actor, HTN for workday/theft/report/notice/investigate/recruit, bounded local planning, relationship trust, lying, and interruption.
