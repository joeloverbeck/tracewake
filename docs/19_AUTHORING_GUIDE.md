# Authoring Guide

## Core rule

Author causes, affordances, norms, procedures, traces, and artifacts. Do not author quests.

A designer creates simulation materials: actions, objects, roles, norms, routines, institutions, traces, needs, motives, procedures, notices, speech-act templates, routes, sites, and domain content.

## Authoring actions

Every action needs actor requirements, physical preconditions, social/legal preconditions, knowledge requirements, cost, duration, effects, traces, failures, observation hooks, and norm implications.

```yaml
Action: SearchContainer
requirements:
  - actor_at_container_location
  - container_accessible
cost:
  time: 30s-5m
  attention: medium
effects:
  - create_observation_of_contents
  - if_expected_item_absent_create_contradiction_candidate
traces:
  - container_disturbed_if_careless
norms:
  - may_violate_privacy_or_property
failures:
  - interrupted
  - too_dark
  - locked
```

## Authoring objects

Objects advertise affordances but do not ignore context.

```yaml
Object: Strongbox
affordances: [Open, Close, Lock, Unlock, ForceOpen, Inspect, RemoveItem, PlaceItem]
social_context:
  legal_access_requires_permission: true
trace_profile:
  forced_open_creates_lock_damage: true
  open_close_may_create_noise: true
```

## Authoring traces

Traces should usually be ambiguous.

```yaml
Trace: LockScratch
possible_causes: [forced_entry, careless_owner, old_damage]
supports:
  ContainerForcedOpen: 0.65
contradicts:
  NormalAccess: 0.25
can_be_altered_by: [RepairLock, ReplaceLock]
```

## Authoring beliefs

Do not write “NPC knows X” unless a source exists.

```yaml
InitialBelief:
  holder: actor_tomas
  proposition: ItemLocatedInContainer(coin_stack_01, strongbox_tomas)
  confidence: 0.86
  source: prior_direct_observation
```

## Authoring routines

Routines are conditional HTN-like methods.

```yaml
RoutineTemplate: MillerWorkday
preconditions:
  - actor_believes_today_is_workday
  - actor_not_too_ill
  - mill_reachable
  - no_stronger_obligation
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

Institution procedures replace quest scripts.

```yaml
Procedure: PostBountyForThreat
trigger:
  - threat_report_received
conditions:
  - threat_within_jurisdiction
  - estimated_risk >= medium
  - funds_available >= reward
steps:
  - create_case_record
  - reserve_funds
  - assign_clerk_write_notice
  - assign_runner_post_notice
failure_modes:
  - insufficient_funds
  - clerk_absent
  - report_not_credible
```

## Authoring speech

Dialogue is generated or selected from beliefs and intentions.

Bad:

```text
"I know Mara stole Tomas's coins."
```

Better if speaker heard rumor:

```yaml
SpeechAct:
  kind: Gossip
  proposition: MaraNearTomasHouse
  source: rumor
  confidence: 0.38
  stance: suspicious
```

## Authoring notices

A notice has structured claims.

```yaml
NoticeTemplate: RoadThreatBounty
fields:
  issuer: institution
  threat_claim: proposition
  believed_location: optional
  reward: amount
  proof_required: list
  last_verified: time
surface_text:
  formal: "By order of {issuer}..."
```

## Authoring regions

For each settlement, site, or route, author identity, ownership/jurisdiction, connections, traffic, institutions, resources, risks, rumor channels, economic flows, likely traces, and default LOD.

## Authoring domain packs

Define added actions, body differences, special information channels, institutions/norms, threats, items, traces, speech/culture tags, and procedural constraints. Do not hard-code flavor into the kernel.

## Converting a traditional quest

Traditional:

```text
Tomas asks player to recover stolen heirloom from bandit cave. Reward 100 coins.
```

Tracewake version:

1. Tomas owns heirloom.
2. Someone steals or raids.
3. Tomas discovers absence.
4. Tomas forms distress/suspicion.
5. He reports or privately requests help.
6. Trace or rumor links item to a fence/bandits.
7. Tomas offers reward if he has funds and believes help is useful.
8. Request/notice/contract is created.
9. Anyone can pursue, ignore, fake, steal, return, ransom, or destroy item.
10. Reward depends on proof, belief, funds, and dispute.

No quest object required.

## Review checklist

What process creates this? Who knows first? How can others learn? What traces exist? What wrong beliefs are plausible? What institution cares? What if player ignores it? What if NPC resolves it? What if information is stale? Can artifact be destroyed, forged, or contradicted? Does any step assume player specialness?

## Authoring priority

Primitive actions, object affordances, belief propositions, trace types, speech acts, routines, institution procedures, notice templates, route/site data, domain flavor, cosmetic prose.
