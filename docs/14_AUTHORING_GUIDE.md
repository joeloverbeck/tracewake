# Authoring Guide

## Core rule

Author causes, affordances, norms, and artifacts. Do not author quests.

A designer’s job is to create materials that the simulation can use:

- actions;
- objects;
- roles;
- norms;
- routines;
- institutions;
- traces;
- needs;
- motives;
- procedures;
- notices;
- dialogue templates grounded in beliefs.

## Authoring actions

Every action needs:

- actor requirements;
- physical preconditions;
- social/legal preconditions;
- knowledge/belief requirements if any;
- cost;
- duration;
- effects;
- traces;
- possible failures;
- observation hooks;
- norm implications.

Example:

```yaml
Action: SearchContainer
requirements:
  - actor_at_container_location
  - container_accessible
cost:
  time: 30s-5m
  attention: medium
effects:
  - create Observation of contents for actor
  - if expected item absent, generate contradiction candidate
traces:
  - container_disturbed if careless
norms:
  - may violate privacy/property if actor lacks permission
failures:
  - interrupted
  - too dark
  - container locked
```

## Authoring objects

Objects should advertise affordances but not ignore context.

```yaml
Object: Strongbox
affordances:
  - Open
  - Close
  - Lock
  - Unlock
  - ForceOpen
  - Inspect
  - RemoveItem
  - PlaceItem
social_context:
  ownership_required_for_legal_access: true
trace_profile:
  forced_open_creates_lock_damage: true
  open_close_may_create_noise: true
```

## Authoring traces

Traces should be ambiguous unless inherently conclusive.

```yaml
Trace: LockScratch
possible_causes:
  - forced_entry
  - careless_owner
  - old_damage
supports:
  ContainerForcedOpen: 0.65
contradicts:
  NormalAccess: 0.25
decay:
  rate: very_slow
can_be_altered_by:
  - RepairLock
  - ReplaceLock
```

## Authoring beliefs

Do not write “NPC knows X” unless a source exists.

Better:

```yaml
InitialBelief:
  holder: actor_tomas
  proposition: ItemLocatedAt(item_coin_stack_01, item_strongbox_01)
  confidence: 0.86
  source: prior_direct_observation
  acquired_at: pre_simulation
```

## Authoring routines

Routine templates should be conditional.

```yaml
RoutineTemplate: MillerWorkday
preconditions:
  - actor believes today is workday
  - actor is not too ill
  - mill reachable
  - no stronger obligation active
steps:
  - wake
  - eat_if_hungry
  - travel_to_mill
  - open_mill
  - process_grain_if_available
  - trade_with_customers
  - record_payments
  - close_mill
  - return_home
interruptions:
  - family_emergency
  - guard_summons
  - road_threat
  - missing_key
```

## Authoring institutions

Institution procedures are the best replacement for quest scripts.

```yaml
Procedure: PostBountyForThreat
trigger:
  - threat_report_received
conditions:
  - threat_within_jurisdiction
  - estimated_risk >= medium
  - funds_available >= reward_amount
steps:
  - create_case_record
  - reserve_funds
  - assign_clerk_to_write_notice
  - assign_actor_to_post_notice
  - update_public_records
failure_modes:
  - insufficient_funds
  - clerk_absent
  - threat_outside_jurisdiction
  - report_not_credible
```

## Authoring notices

A notice must have structured claims.

```yaml
NoticeTemplate: BanditBounty
fields:
  issuer: institution
  threat_claim: proposition
  believed_location: optional location
  reward: amount
  proof_required: list
  expiry_or_review_date: optional
  last_verified: time
surface_text:
  formal: "By order of {issuer}..."
  terse: "Bandits reported near {location}. Reward: {reward}."
```

Generated prose should never be the only location of the facts.

## Authoring rumors

Rumors need:

- origin type;
- mutation profile;
- social charge;
- confidence behavior;
- propagation contexts.

```yaml
RumorProfile: CrimeSuspicion
spreads_at: [tavern, market, queue, workplace]
mutation_modes: [exaggerate, identity_confuse, motive_invent]
confidence_drift: upward_socially_downward_factually
social_charge: high
```

## Authoring dialogue

Dialogue should be generated or selected from actor beliefs and intentions.

Bad:

```text
“I know Mara stole Tomas's coins.”
```

Better if the speaker only heard rumor:

```text
“I heard Mara was near Tomas's place last night. Could be nothing.”
```

Structured payload:

```yaml
SpeechAct:
  proposition: MaraNearTomasHouse
  source: rumor
  confidence: 0.38
  stance: suspicious
```

## Converting a traditional quest

Traditional quest:

```text
Tomas asks player to recover stolen heirloom from bandit cave. Reward 100 coins.
```

Causality-first version:

1. Tomas owns heirloom and stores it at home.
2. Bandits raid or thief steals and later sells it.
3. Tomas discovers absence because he checks storage.
4. Tomas forms distress and suspicion.
5. Tomas reports or privately asks trusted people.
6. A rumor or trace links item to fence/bandits.
7. Tomas can offer reward if he has money and believes outside help is useful.
8. A request or notice is created.
9. Anyone can pursue, ignore, fake, steal, return, ransom, or destroy the item.
10. Reward depends on Tomas believing proof and still having funds.

No quest object required.

## Review checklist

Before merging authored content, ask:

- What world process creates this?
- Who knows about it initially?
- How can others learn?
- What traces exist?
- What wrong beliefs are plausible?
- What institution, if any, cares?
- What happens if the player ignores it?
- What happens if another NPC resolves it?
- What happens if information is stale?
- Can the notice/record/request be destroyed, forged, or contradicted?
- Does any step assume the player is special?

## Anti-patterns for authors

- Writing a quest giver monologue before modeling the problem.
- Creating a reward without custody.
- Spawning enemies because a task requires enemies.
- Making NPCs wait indefinitely for the player.
- Hiding all uncertainty from objective text.
- Creating “fake” buildings/records/containers that cannot be interacted with.
- Treating rumors as true because they are convenient.

## Authoring priority for prototype

Author in this order:

1. primitive actions;
2. object affordances;
3. belief propositions;
4. trace types;
5. routines;
6. institution procedures;
7. notice templates;
8. rumor profiles;
9. dialogue surfaces;
10. cosmetic flavor.

Do not reverse this order.
