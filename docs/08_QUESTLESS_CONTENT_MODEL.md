# Questless Content Model

## Core claim

A quest should not be an engine object. It is a user-interface interpretation of world processes.

The engine should store things like:

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
- sanctions.

The player may see some of these as tasks. The world does not.

## Why abolish quests

Traditional quests smuggle in assumptions:

- the player is the intended solver;
- the world waits;
- the objective is objectively true;
- reward is guaranteed;
- the target exists and is relevant;
- completion is globally recognized;
- quest state is more authoritative than simulation state.

Those assumptions are poison for this project.

## Replacement ontology

### Incident

Something happened that may matter.

```yaml
Incident:
  id: incident_bandit_attack_03
  kind: road_attack
  source_events: [evt_attack_882]
  known_by:
    - actor_survivor
  affected_parties:
    - actor_survivor
    - merchant_guild
  status_by_truth: bandits_alive_unknown_to_public
  public_visibility: low
```

### Need

An actor or institution lacks something.

```yaml
Need:
  holder: inst_reeves_office
  kind: reduce_road_threat
  urgency: 0.82
  source: incident_bandit_attack_03
```

### Request

A communication asking someone to do something.

```yaml
Request:
  requester: actor_tomas
  target: actor_guard_elias
  desired_outcome: recover_missing_coins
  offered_reward: none
  status: made
```

### Contract

A formal agreement with obligations and verification.

```yaml
Contract:
  issuer: inst_reeves_office
  open_to: any_licensed_adventurer
  desired_outcome: neutralize_bandit_group_north_road
  reward: 80
  proof_required: [token, witness, recovered_goods]
  funds_reserved: true
  status: active
```

### Notice

A public artifact that communicates a request, warning, law, rumor, or record.

```yaml
Notice:
  written_by: actor_anna_clerk
  authorized_by: actor_reeve
  content_ref: notice_text_998
  structured_claims:
    - bandits_threaten_north_road
    - reward_offered_80
  posted_at: board_market_square
  posted_time: day_144_10_30
  status: physically_posted
```

### Opportunity

A situation that an agent could exploit.

```yaml
Opportunity:
  kind: unguarded_valuable
  observed_by: actor_mara
  target: item_coin_stack_01
  risk_estimate: low
```

### Obligation

A duty created by role, promise, law, contract, kinship, debt, or oath.

```yaml
Obligation:
  obligated_actor: actor_guard_elias
  owed_to: inst_reeves_office
  action: patrol_north_road
  source: role_guard
  violation_if: absent_without_cause
```

## Player-facing task card

The UI may show a task card, but it should admit epistemic uncertainty.

Bad:

```text
Quest: Kill the Bandits at North Cave. Reward: 80g.
```

Better:

```text
Posted Notice: Road Bandits Reported
Issuer: Reeve's Office
Claim: travelers report attacks on the north road
Believed location: somewhere near North Cave
Reward: 80 coins, if proof satisfies the reeve
Last verified: 2 days ago
Known uncertainty: threat may have moved
```

The second version is not just flavor. It tells the player that the notice is a fallible artifact.

## Content lifecycle: bandit bounty

```text
Harm -> observation -> report -> institutional belief -> response decision
-> contract -> notice -> reader belief -> candidate intention -> action
-> proof -> verification -> payment/refusal/update/removal
```

Every arrow should correspond to events.

## Content lifecycle: missing coins

```text
Coins stored -> thief observes opportunity -> theft -> owner expects coins
-> owner checks container -> contradiction -> suspicion -> search/report/accuse
-> case file -> investigation -> rumor -> recovery/wrong accusation/stale case
```

No quest is needed.

## Reward model

Rewards should be real obligations with custody.

Questions:

- Who promised the reward?
- Do they have funds?
- Are funds reserved?
- What proof is required?
- Who can verify proof?
- Is the payer still alive/in office/solvent?
- Can someone else claim credit?
- Can proof be forged?
- Can the institution refuse payment?
- Can the player enforce payment?

This creates more gameplay than a guaranteed reward screen.

## Completion model

Replace `quest.complete = true` with world-specific resolutions.

Examples:

```yaml
ThreatResolution:
  threat: incident_bandit_attack_03
  truth_state: bandits_dead
  known_by_authority: false
  public_notice_status: stale
  trade_route_risk_actual: low
  trade_route_risk_believed: high
```

```yaml
ContractResolution:
  contract: bounty_07
  claimant: actor_player_body_current
  proof_submitted: item_bandit_token
  verifying_actor: actor_reeve
  accepted: true
  payment_event: evt_paid_bounty_07
```

The threat can be solved while the contract remains unresolved. The contract can be paid while rumors remain wrong.

## Generated text

Text generation should take structured facts and produce readable artifacts.

Input:

```yaml
issuer: Reeve's Office
claim: bandits attacked two travelers on north road
confidence: medium
reward: 80 coins
proof: token or witness
last_verified: day_144
```

Output:

```text
By order of the Reeve: armed brigands are reported on the north road. Payment of 80 coins will be made for proof of their dispersal or death. Present proof to the office before sundown market day.
```

The generated text is not authoritative. The structured payload is.

## Hooks without quests

Interesting hooks emerge from artifacts:

- a blood trail;
- a locked door left open;
- an overheard accusation;
- a stale notice;
- a sudden price spike;
- a closed shop;
- a missing worker;
- a guard asking questions;
- a debtor hiding;
- a funeral;
- a rumor that contradicts a record.

The UI can collect these as “leads,” not quests.

## Anti-patterns

- `GenerateQuestForPlayer()`.
- “Quest target” known by UI regardless of actor knowledge.
- Reward spawned on completion.
- Quest giver waiting forever.
- Dead target respawned because quest requires it.
- Problem pauses until accepted.
- Objective text describing ground truth rather than issuer belief.
- Stale information cleaned up automatically for player convenience.

## Minimal questless implementation

For the first village, implement:

- incident records;
- public notices;
- requests from one actor to another;
- open bounty contract;
- proof requirements;
- stale notice state;
- player lead UI;
- institutional verification;
- removal/update of notice by physical action.

That is enough to replace quests in the prototype.
