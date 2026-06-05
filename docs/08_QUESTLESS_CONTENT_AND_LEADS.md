# Questless Content and Leads

## Core claim

A quest is not an engine object. It is a player-interface interpretation of world processes.

The engine stores incidents, needs, threats, requests, contracts, obligations, debts, rumors, records, notices, opportunities, promises, investigations, and sanctions.

## Why abolish quests

Traditional quests imply the player is intended solver, the world waits, objectives are objectively true, reward is guaranteed, targets exist for the quest, completion is globally recognized, and quest state outranks simulation state. Tracewake rejects all of that.

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
```

### Contract

```yaml
Contract:
  issuer: reeves_office
  open_to: licensed_adventurers
  desired_outcome: neutralize_road_threat
  reward: 80
  proof_required: [token, witness, recovered_goods]
  funds_reserved: true
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
```

### Opportunity

```yaml
Opportunity:
  kind: unguarded_valuable
  observed_by: actor_mara
  target: coin_stack_01
  risk_estimate: low
```

## Leads

The UI tracks leads, not quests. A lead is source-bound, fallible, actor-aware, possibly stale, not guaranteed solvable, and not necessarily rewarding.

Example:

```yaml
LeadCard:
  title: Road threat reported
  source: notice_bandits_07
  known_to_current_actor: true
  claims:
    - travelers report attacks on north road
    - reeve offers 80 coins for proof
  uncertainty:
    - last verified two days ago
    - threat location uncertain
```

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
attack -> observation -> survival/travel -> report -> institutional belief
-> response decision -> contract -> notice -> reader belief -> recruitment
-> travel -> discovery/failure/stale lead -> proof -> verification/payment/refusal
```

## Reward model

Rewards are real obligations with custody. Ask who promised, whether funds exist, what proof is required, who verifies, whether payer is alive/in office/solvent, whether proof can be forged, and whether the claimant can enforce payment.

## Completion model

Replace `quest.complete = true` with domain resolution.

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

## Hooks without quests

Blood trail, locked door left open, overheard accusation, stale notice, price spike, closed shop, missing worker, guard questioning, debtor hiding, funeral, rumor contradicting record, unpaid adventurers, abandoned cart.

## Anti-patterns

`GenerateQuestForPlayer()`, reward spawned on completion, problem pauses until accepted, objective text describing ground truth, stale information cleaned automatically, NPCs waiting indefinitely.
