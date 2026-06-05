# Questless Leads, Notices, and Story Sifting

## Status

This document defines questless content architecture, lead projections, notices, records, incidents, opportunities, and observer-only story sifting.

A quest is not an engine object in Tracewake.

## Core rule

The engine stores world processes and artifacts. The UI may project source-bound leads from those processes. The projection must never become objective quest truth.

Engine ontology includes:

- incidents;
- needs;
- requests;
- contracts;
- obligations;
- debts;
- records;
- notices;
- rumors;
- opportunities;
- investigations;
- suspicions;
- sanctions;
- traces;
- institutional procedures;
- household concerns.

It does not include objective quest progress, quest targets, quest completion flags, or player-exclusive objectives.

## Why quests are forbidden as ontology

Traditional quest ontology implies:

- the player is the intended solver;
- the world waits;
- objectives are objectively true;
- rewards are guaranteed;
- targets exist for the quest;
- completion is globally recognized;
- quest state outranks simulation state;
- NPCs exist to serve the player's arc.

Tracewake rejects all of these.

## Incidents

An incident is something that happened or is believed to have happened and may matter.

```yaml
Incident:
  id: incident_missing_coins_01
  kind: missing_property
  source_events:
    - evt_tomas_found_strongbox_empty
  affected_parties:
    - actor_tomas
    - household_tomas
  known_by:
    - actor_tomas
  institutional_status: unreported
  public_visibility: none
```

An incident is not a quest. It may never become known to the human-controlled actor.

## Requests

A request is a speech/social action asking someone to do something.

```yaml
Request:
  id: request_tomas_to_clerk_help_find_coins
  requester: actor_tomas
  target: actor_anna_clerk
  desired_outcome: recover_or_record_missing_property
  source_beliefs:
    - belief_tomas_coins_missing
  offered_reward: none
  status: received_or_refused_or_ignored
```

Requests can be refused, misunderstood, delayed, fulfilled by someone else, or become stale.

## Contracts and obligations

A contract or obligation is a social/institutional artifact with parties, terms, authority, proof/payment rules if any, and lifecycle events.

First slice may need simple promises or debts. Bounties/rewards are second-proof material.

```yaml
Obligation:
  id: debt_mara_iva_01
  debtor: actor_mara
  creditor: actor_iva
  amount: small_coin_sum
  due: market_day
  source_record_or_memory: debt_note_or_speech_event
  enforcement: social_pressure
```

## Notices

A notice is a public artifact.

```yaml
Notice:
  id: notice_missing_property_01
  kind: public_notice
  written_by: actor_anna_clerk
  authorized_by: reeves_office
  structured_claims:
    - TomasReportsMissingCoins
    - LastExpectedInStrongbox
    - AnyoneWithInformationMaySpeakToClerk
  posted_at: notice_board_village_square
  posted_event: evt_notice_posted_001
  last_verified: 142-08-12T10:00
  status: posted
```

A notice can be stale, forged, ignored, destroyed, contradicted, copied, removed, misread, solved by someone else, or never acted on.

## Leads

A lead is a UI projection over actor-known, source-bound information.

```yaml
LeadCard:
  id: lead_missing_coins_for_tomas
  viewer: actor_tomas
  source:
    kind: expectation_contradiction
    id: evt_tomas_found_strongbox_empty
  title: Missing coins from strongbox
  actor_known_claims:
    - coins should have been in strongbox
    - strongbox is now empty
    - Elena says she heard a noise near the room
  uncertainty:
    - no direct witness
    - coins may have been moved by someone permitted
    - memory could be wrong
  suggested_actions:
    - search room
    - ask household
    - report to clerk
    - inspect for traces
```

Suggested actions are actor-possible action projections. They do not create objectives or progress bars.

## Lead projection rules

Lead cards must be:

- source-bound;
- actor-filtered;
- uncertainty-preserving;
- stale-aware;
- non-authoritative;
- rebuildable from events/beliefs/records;
- removable when actor forgets, loses access, contradicts, resolves, or no longer cares;
- inspectable in debug.

Lead cards must not:

- reveal ground truth;
- imply guaranteed reward;
- imply player ownership;
- freeze the world;
- mark objective completion;
- point to actual culprit or hidden item unless actor knows;
- spawn clues;
- make NPCs wait.

## Player-facing wording

Bad:

```text
Quest: Find Tomas's Stolen Coins
Objective: Accuse Mara
Reward: 10 coins
```

Good:

```text
Lead: Missing coins from strongbox
Source: Your own search of the strongbox
Known claims: The coins should have been there. They were not there when checked.
Related claim: Elena says she heard a noise near the bedroom last night.
Uncertainty: No one you know saw the coins taken.
Possible actions: Search nearby, ask household, report to clerk.
```

## Story sifting

Story sifting is an observer layer that recognizes salient patterns after events occur.

Allowed early use:

- no-human simulation recap;
- debug summary of causal chains;
- grouping related events;
- highlighting contradictions;
- surfacing wrong beliefs;
- identifying unresolved incidents;
- explaining stale notices;
- helping developers inspect emergent behavior;
- later supporting actor-filtered summaries.

Forbidden uses:

- spawning incidents;
- selecting culprits;
- creating clues;
- repairing pacing;
- moving NPCs for drama;
- making NPCs wait for the player;
- revealing hidden truth in embodied mode;
- converting incidents into quests;
- altering probabilities because the player is bored.

Story sifting answers, "what pattern occurred?" It never answers, "what must happen next?"

## Minimal observer sifter

Include a minimal observer/debug sifter early if useful.

Input:

```text
event log + causal graph + projections + actor beliefs + records + traces
```

Output:

```yaml
StorySiftSummary:
  mode: debug_observer
  chain: missing_property_chain_01
  salient_events:
    - item_expected_in_strongbox
    - item_removed_or_moved
    - expectation_contradiction
    - report_received
  contradictions:
    - Tomas suspects Mara without direct evidence
    - Clerk record omits uncertainty about noise
  unresolved:
    - coin location unknown to Tomas
    - institutional proof insufficient
  hidden_truth_allowed: true_in_debug_only
```

Player-facing story sifting is deferred and must be actor-filtered.

## Missing-property leads

The first proof may generate leads from:

- expected property absent;
- disturbed container trace;
- heard noise;
- household member refusal;
- gossip about debt;
- clerk ledger entry;
- notice board item;
- contradiction between record and rumor;
- discovered hidden property;
- apology/confession/lie.

Each lead must name its source and uncertainty.

## Road-threat/bounty as second proof

Road-threat/bounty/expedition flow is valuable but deferred.

When implemented, the flow is:

```text
harm or threat process
 -> observation/survivor/report
 -> institutional belief
 -> resource/jurisdiction/procedure decision
 -> contract or notice artifact
 -> readers form beliefs
 -> agents may act or ignore/refuse
 -> proof submitted
 -> proof interpreted/disputed/refused
 -> payment, partial payment, fraud accusation, stale closure, or no resolution
```

At no point is this a quest primitive.

## Completion replacement

Replace quest completion with domain state.

```yaml
MissingPropertyResolution:
  item: coin_stack_01
  actual_location: hidden_from_embodied_unless_known
  owner_belief: still_missing
  institution_belief: report_open_insufficient_evidence
  public_rumor: mara_might_have_taken_it
  actual_possession: actor_mara
  notice_status: none
```

Different layers can resolve differently. That divergence is the point.

## Acceptance implications

Questless/lead features must test:

- no quest state drives truth;
- lead source exists and is actor-known;
- lead suggested actions are action-registry proposals;
- stale notices remain stale until updated;
- someone else can resolve or change situation;
- story sifter cannot create events;
- embodied summaries hide hidden truth;
- debug summaries expose causal graph;
- no reward appears without custody/obligation/procedure;
- no NPC waits for player acceptance.

## Anti-patterns

- `GenerateQuestForPlayer`.
- Objective text describes ground truth.
- Completion flag decides institutional belief.
- Reward spawned on completion.
- Notice automatically removed when truth changes.
- Marker points to real culprit from rumor.
- Story sifter becomes director.
- Problem pauses until accepted.
- Lead card shows debug truth in embodied mode.
- Hidden quest state wrapped in questless vocabulary.
