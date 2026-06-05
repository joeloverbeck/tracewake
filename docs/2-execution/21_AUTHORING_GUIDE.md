# Authoring Guide

This guide is the worked how-to of the `0-foundation/05_NO_SCRIPTING_AND_CAUSAL_AUTHORING_POLICY.md` policy (itself the expansion of `INV-038` and `INV-035`–`INV-037`). The policy says what authored machinery is allowed versus forbidden; this guide shows how to write it. Where the two diverge, the policy governs.

## Core rule

Author causes, affordances, norms, procedures, traces, and artifacts. Do not author quests.

A designer creates simulation materials:

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
- speech-act templates;
- routes;
- sites;
- domain content;
- scenario seeds;
- test fixtures.

## Authoring actions

Every action needs:

- actor requirements;
- physical preconditions;
- social/legal preconditions;
- knowledge requirements;
- resource requirements;
- cost;
- duration;
- effects;
- traces;
- failures;
- observation hooks;
- norm implications;
- TUI-facing why-not reasons.

```yaml
Action: SearchContainer
requirements:
  - actor_at_container_location
  - container_accessible
knowledge:
  - actor_knows_or_can_perceive_container
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
  - refuses_to_trespass
```

## Authoring objects

Objects advertise affordances but do not ignore context.

```yaml
Object: Strongbox
affordances:
  - Open
  - Close
  - Lock
  - Unlock
  - ForceOpen
  - Inspect
  - Search
  - RemoveItem
  - PlaceItem
social_context:
  legal_access_requires_permission: true
trace_profile:
  forced_open_creates_lock_damage: true
  open_close_may_create_noise: true
  careful_search_may_reduce_disturbance: true
```

An affordance is not permission. Permission is evaluated through actor state, ownership, norms, willingness, and context.

## Authoring traces

Traces should usually be ambiguous.

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
visibility: medium
can_be_altered_by:
  - RepairLock
  - ReplaceLock
  - PlantFalseScratch
```

Avoid perfect clues unless the domain has a causal reason.

## Authoring beliefs

Do not write “NPC knows X” unless a source exists.

```yaml
InitialBelief:
  holder: actor_tomas
  proposition: ItemLocatedInContainer(coin_stack_01, strongbox_tomas)
  confidence: 0.86
  source:
    kind: prior_direct_observation
    source_id: seed_evt_tomas_checked_strongbox
  acquired_at: seed_time
```

For initial state, seed events or explicit authored initial-belief sources are acceptable. They must be inspectable.

## Authoring expectations

Expectations make absence meaningful.

```yaml
Expectation:
  holder: actor_tomas
  proposition: ItemLocatedInContainer(coin_stack_01, strongbox_tomas)
  check_contexts:
    - morning_accounting
    - preparing_tax_payment
  contradiction_response:
    - surprise
    - search
    - ask_household
    - report_if_value_high
```

## Authoring routines

Routines are conditional HTN-like methods.

```yaml
RoutineTemplate: MillerWorkday
preconditions:
  - actor_believes_today_is_workday
  - actor_not_too_ill
  - actor_has_or_can_get_breakfast
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
  - road_threat_believed_high
  - missing_key
  - fatigue_too_high
```

A routine is invalid if it cannot fail.

## Authoring institutions

Institution procedures replace quest scripts.

```yaml
Procedure: PostBountyForThreat
trigger:
  - credible_threat_report_received
conditions:
  - threat_within_jurisdiction
  - estimated_risk >= medium
  - funds_available >= reward
  - clerk_available_or_can_delay
steps:
  - create_case_record
  - reserve_funds
  - define_proof_required
  - assign_clerk_write_notice
  - assign_runner_post_notice
failure_modes:
  - insufficient_funds
  - clerk_absent
  - report_not_credible
  - jurisdiction_disputed
  - notice_destroyed_before_posting
```

Ask what happens when the procedure fails. Failure is content.

## Authoring norms

Norms should specify classification and social machinery.

```yaml
Norm: NoTheft
kind: prohibition
action_pattern: TakePropertyWithoutPermission
conditions:
  - item_has_legal_owner
  - actor_lacks_permission
  - actor_not_authorized_by_exception
violation_event: TheftViolation
detection_channels:
  - direct_observation
  - missing_item_expectation
  - trace_interpretation
  - confession
  - witness_testimony
sanctions:
  - return_property
  - fine
  - detention_if_severe
```

Violation does not imply detection.

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
  source: rumor_chain_07
  confidence: 0.38
  stance: suspicious
```

If the speaker lies, author the motive and risk.

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

The text is a rendering. The claims and provenance are the artifact.

## Authoring regions

For each settlement, site, or route, author:

- identity;
- ownership/jurisdiction;
- connections;
- travel costs;
- traffic;
- institutions;
- resources;
- risks;
- rumor channels;
- economic flows;
- likely traces;
- default LOD;
- known public beliefs;
- stale records if any.

## Authoring domain packs

Define:

- body differences;
- special information channels;
- institutions/norms;
- threats;
- items;
- traces;
- diseases/conditions;
- speech/culture tags;
- procedural constraints;
- scenario seeds.

Do not hard-code flavor into the kernel.

## Converting a traditional quest

Traditional:

```text
Tomas asks player to recover stolen heirloom from bandit cave. Reward 100 coins.
```

Tracewake version:

1. Tomas owns heirloom.
2. Someone steals, raids, borrows, confiscates, or misplaces it.
3. Tomas discovers absence through expectation/search.
4. Tomas forms distress/suspicion.
5. He reports or privately requests help.
6. Trace, rumor, record, or inference links item to a person/place/group.
7. Tomas offers reward if he has funds and believes help is useful.
8. Request, contract, notice, or rumor is created.
9. Anyone can pursue, ignore, fake, steal, return, ransom, destroy, or contest the item.
10. Reward depends on proof, belief, funds, and dispute.

No quest object required.

## Scenario seed checklist

A good seed defines:

- starting state;
- relevant beliefs;
- pressures;
- relationships;
- possessions;
- locations;
- institutions;
- records;
- routines;
- uncertainty;
- possible interruptions;
- no guaranteed outcome.

A bad seed defines the intended plot.

## Review checklist

Ask:

```text
What process creates this?
Who knows first?
How can others learn?
What traces exist?
What wrong beliefs are plausible?
What institution cares?
What if player ignores it?
What if an NPC resolves it?
What if information is stale?
Can artifact be destroyed, forged, contradicted, or lost?
Can every world-affecting action be attempted by an NPC?
Can the TUI expose it without leaking truth?
Does any step assume player specialness?
Does any step require LLM authority?
Does any step assume low-fantasy kernel rules?
```

## Authoring priority

1. primitive actions;
2. object affordances;
3. belief proposition vocabulary;
4. trace types;
5. speech acts;
6. needs/routines;
7. ownership/storage;
8. institution procedures;
9. notice/record templates;
10. route/site data;
11. domain flavor;
12. cosmetic prose.

## Anti-patterns

- quest beat hidden as procedure;
- object affordance that ignores ownership;
- clue with only one interpretation;
- NPC knowledge without source;
- notice without author/issuer;
- reward without funds;
- routine without interruption;
- domain pack bypassing core action pipeline;
- prose-only history;
- LLM-generated facts.
