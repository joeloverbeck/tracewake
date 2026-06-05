# Questless Content, Leads, and Story Sifting

## Core claim

A quest is not an engine object. It is a player-interface interpretation of source-bound world processes.

The engine stores:

- incidents;
- needs;
- threats;
- requests;
- contracts;
- obligations;
- debts;
- rumors;
- records;
- notices;
- opportunities;
- promises;
- investigations;
- sanctions;
- social tensions;
- traces.

## Why abolish quests

Traditional quests imply:

- the player is intended solver;
- the world waits;
- objectives are objectively true;
- reward is guaranteed;
- target exists for the quest;
- completion is globally recognized;
- quest state outranks simulation state;
- NPCs exist to serve the player's arc.

Tracewake rejects all of that.

## Replacement ontology

### Incident

Something happened that may matter.

```yaml
Incident:
  id: incident_road_attack_03
  kind: road_attack
  source_events: [evt_attack_882]
  affected_parties: [actor_survivor, merchant_guild]
  known_by: [actor_survivor]
  public_visibility: low
```

### Need

```yaml
Need:
  holder: reeves_office
  kind: reduce_road_threat
  urgency: 0.82
  source: incident_road_attack_03
```

### Request

```yaml
Request:
  requester: actor_tomas
  target: actor_elias_guard
  desired_outcome: recover_missing_coins
  offered_reward: none
  source_beliefs:
    - belief_tomas_coins_missing
```

### Contract

```yaml
Contract:
  issuer: reeves_office
  open_to: licensed_adventurers
  desired_outcome: reduce_road_threat
  reward: 80
  proof_required: [token, witness, recovered_goods]
  funds_reserved: true
  record: bounty_ledger_05
```

### Notice

```yaml
Notice:
  written_by: actor_anna_clerk
  authorized_by: actor_reeve
  structured_claims:
    - travelers_report_attacks_on_north_road
    - reward_offered_80
  posted_at: market_notice_board
  last_verified: 142-08-10T13:40
```

### Opportunity

```yaml
Opportunity:
  kind: unguarded_valuable
  observed_by: actor_mara
  target: coin_stack_01
  risk_estimate: low
  source: observed_open_strongbox
```

## Leads

The UI tracks leads, not quests. A lead is:

- source-bound;
- fallible;
- actor-aware;
- possibly stale;
- not guaranteed solvable;
- not necessarily rewarding;
- not restricted to the player;
- not proof of truth.

Example:

```yaml
LeadCard:
  title: Road threat reported
  source: notice_bandits_07
  known_to_current_actor: true
  claims:
    - travelers report attacks on north road
    - reeve offers 80 coins for accepted proof
  uncertainty:
    - last verified two days ago
    - threat location approximate
    - notice may be stale
  suggested_actions:
    - ask locals
    - inspect office record if permitted
    - recruit companion
    - travel north
```

Suggested actions are UI projections over actor-possible actions. They do not create objectives.

## Player-facing wording

Bad:

```text
Quest: Kill the Bandits at North Cave. Reward: 80g.
```

Better:

```text
Posted Notice: Road Threat Reported
Issuer: Reeve's Office
Claim: travelers report attacks on the north road
Believed location: near old quarry
Reward: 80 coins if proof satisfies the reeve
Last verified: two days ago
```

## Content lifecycle: road threat

```text
attack -> observation -> survivor travel -> report -> institutional belief
-> response decision -> contract -> notice -> reader belief -> recruitment
-> travel -> discovery/failure/stale lead -> proof -> verification/payment/refusal
```

Every arrow is a modeled process.

## Reward model

Rewards are real obligations with custody.

Ask:

- who promised;
- what institution or person authorized it;
- whether funds exist;
- where funds are held;
- what proof is required;
- who verifies proof;
- whether proof can be forged;
- whether the payer remains alive/in office/solvent;
- whether another claimant contests;
- whether claimant can enforce payment.

## Completion model

Replace:

```yaml
quest.complete: true
```

with domain resolution:

```yaml
ThreatResolution:
  threat: incident_road_attack_03
  truth_state: bandits_dead
  authority_belief: unresolved
  notice_status: stale
  route_actual_risk: low
  route_believed_risk: high
```

The threat can be solved while the contract remains unresolved.

## Story sifting

Story sifting is an observer layer that recognizes salient patterns in event history.

Allowed uses:

- highlight contradiction chains;
- summarize causal arcs;
- identify unresolved incidents;
- group related events;
- surface ironic wrong beliefs;
- produce recaps;
- help debug;
- annotate player notebooks with actor-known sources;
- identify “interesting” no-player events after they occur.

Forbidden uses:

- spawn incidents;
- inject pacing events;
- reveal objective truth in embodied mode;
- repair stale leads;
- guarantee narrative beats;
- select culprits;
- move NPCs for drama;
- make content wait.

Story sifting answers, “what pattern occurred?” It never answers, “what must happen next?”

## Hooks without quests

Examples of questless hooks:

- blood trail;
- locked door left open;
- overheard accusation;
- stale notice;
- price spike;
- closed shop;
- missing worker;
- guard questioning someone;
- debtor hiding;
- funeral;
- rumor contradicting record;
- unpaid adventurers;
- abandoned cart;
- forged ledger;
- empty bed;
- repeated absence from work;
- destroyed notice;
- visitor asking too many questions.

## Anti-patterns

- `GenerateQuestForPlayer()`;
- reward spawned on completion;
- problem pauses until accepted;
- objective text describes ground truth;
- stale information cleaned automatically;
- NPCs waiting indefinitely;
- story sifter becomes director;
- marker points to actual culprit;
- player-specific contract resolution;
- “questless” wrapper around hidden quest state.
