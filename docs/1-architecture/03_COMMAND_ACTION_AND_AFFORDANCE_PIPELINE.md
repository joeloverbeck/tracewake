# Command, Action, and Affordance Pipeline

## Status

This document is the architecture contract for every world-affecting action in Tracewake.

The same pipeline is used for human-controlled actors and AI-controlled actors. There are no player-only world verbs.

## Core rule

A world-affecting change is not allowed because the UI asked for it, because a content file scripted it, because an LLM described it, or because a debug tool was convenient.

A world-affecting change is allowed only when a valid actor or process submits a proposal that passes the action pipeline, is scheduled/resolved if required, and commits events.

## Terms

### Command

A command is an input or request from a boundary.

Examples:

```yaml
Command:
  source: human_controller
  bound_actor: actor_tomas
  intent: open_container
  target: strongbox_tomas
```

```yaml
Command:
  source: agent_planner
  actor: actor_mara
  intent: remove_item_from_container
  target: strongbox_tomas
  item: coin_stack_01
```

Commands are not world facts. Commands are proposals waiting to be interpreted, validated, and possibly rejected.

### Action definition

An action definition is an authored or core-defined rule for a kind of possible world transition.

It defines parameters, preconditions, duration, costs, reservation requirements, failure modes, outcome resolution, traces, observation hooks, norm hooks, and event kinds.

### Affordance

An affordance is a typed capability exposed by an entity, place, role, record, social context, or process.

Examples:

```text
door exposes: open, close, lock, unlock, force, listen_through
bed exposes: sleep, nap, inspect_occupied, make_bed
container exposes: open, close, lock, unlock, inspect_contents, remove_item, place_item, search
notice_board exposes: read_notice, post_notice, remove_notice, deface_notice, copy_notice
record exposes: read_record, enter_report, amend_entry, forge_entry, destroy_page
person exposes: greet, ask, answer, report, accuse, lie, refuse, promise, command, testify
road exposes: travel, wait, inspect_tracks, ask_about_route, avoid_route
workplace exposes: begin_work, pause_work, close_shop, sell_item, record_debt
```

Affordances produce candidate actions. They do not bypass validation.

### Proposal

A proposal is a fully parameterized candidate action produced from a command, planner decision, routine, institution procedure, or environment process.

### Rejection

A rejection means the action never began because a required condition was not satisfied. Rejections may be shown as why-not explanations. They become events only when future reasoning can care about the failed attempt.

### Failure

A failure means the action began or became socially/causally meaningful, then did not achieve the intended outcome. Failures are event-worthy when visible, remembered, or causally relevant.

Examples: interrupted theft, failed lock attempt, refused report, item not found during search, accused actor denies claim.

## Unified pipeline

Every world-affecting proposal passes through this pipeline:

```text
1. command/proposal intake
2. actor/process authority check
3. action definition lookup
4. affordance binding
5. parameter resolution
6. physical precondition check
7. knowledge/belief precondition check
8. social/normative precondition check
9. resource/cost/duration check
10. reservation/conflict check
11. risk/willingness check
12. schedule or immediate resolution
13. interruption window checks
14. outcome resolution
15. state mutation plan
16. trace creation plan
17. observation pass
18. belief/projection/reaction hooks
19. institutional/norm hooks
20. event commit
21. projection/view-model update
22. follow-up scheduling
```

No client may skip steps. Some steps may be vacuous for simple actions, but they remain conceptually present.

## Action definition shape

Pseudo-structure:

```yaml
ActionDefinition:
  id: remove_item_from_container
  actor_requirements:
    body_capabilities: [can_reach, can_carry]
    role_permissions: optional
  parameters:
    container: EntityRef<Container>
    item: EntityRef<Item>
  afforded_by:
    entity_components: [Container]
  preconditions:
    physical:
      - actor_can_access_container_location
      - container_allows_open_or_actor_can_attempt_open
      - item_present_or_actor_searching
      - actor_can_carry_item
    knowledge:
      - actor_perceives_item OR actor_has_search_intention
      - target_location_known_or_discoverable
    social_normative:
      - actor_has_permission OR actor_willing_to_violate_property_norm
    resource:
      - actor_has_time_before_required_commitment OR actor_accepts_risk
  reservation:
    locks:
      - actor_body
      - container_access
      - item_possession
  duration:
    base: short
    modifiers: [lock_state, lighting, stealth, fatigue]
  costs:
    fatigue: small
    attention: medium
    time: variable
  possible_outcomes:
    - item_removed
    - item_not_found
    - interrupted
    - caught_in_act
    - container_damaged
  traces:
    - disturbed_container
    - possible_noise
    - possible_missing_item_absence_trace
  observation_hooks:
    sight: local_line_of_sight
    sound: room_sound_propagation
  norm_hooks:
    - property_violation_candidate
    - trespass_violation_candidate
  events:
    started: ActionStarted
    committed: ItemRemovedFromContainer
    failed: ActionFailed
```

This is not implementation code. It is a contract for data and validation.

## Actor authority

An actor can propose an action only if it has authority to attempt it in world terms.

Authority includes:

- body capability;
- consciousness or agency state;
- possession/control binding if the command came from a human;
- role permissions if the action depends on a role;
- physical access;
- social authority or willingness to violate norms;
- knowledge sufficient to form the attempt;
- process authority for non-agent processes such as fire spread, disease, weather, or regional boundary events.

Human control gives input authority only. It never gives world authority.

## Knowledge preconditions

Actors act from beliefs, perception, memory, records, and inference.

An actor may act under uncertainty. Knowledge preconditions are not truth preconditions. They answer:

- Does the actor know or believe the target exists?
- Does the actor know enough to reach or search for it?
- Is the actor speculating, guessing, following a rumor, or acting from a record?
- Is the action a search whose purpose is to discover whether a fact is true?
- Would attempting the action leak ground truth?

Bad:

```text
Tomas can choose "accuse Mara" because the debug log knows Mara stole the coins.
```

Good:

```text
Tomas can choose "accuse Mara" if Tomas has actor-known basis:
Mara was nearby, Mara has motive, someone reported seeing Mara, or Tomas chooses to make a reckless accusation.
```

## Social and normative preconditions

Norms do not simply forbid action. They shape whether an actor is permitted, obligated, deterred, willing to violate, or exposed to later consequences.

Example:

```yaml
SocialNormativeCheck:
  norm: household_privacy
  action: enter_private_room
  result:
    permitted: false
    attempt_allowed_if_willing_to_violate: true
    visible_violation_if_observed: trespass_candidate
    expected_reactions: [confront, report, remember]
```

The architecture must separate:

```text
violation -> detection -> suspicion -> report -> record -> proof -> sanction
```

## Reservations and conflicts

Actions with duration reserve resources to prevent impossible overlaps.

Reservations may cover:

- actor body;
- item possession;
- container access;
- door state;
- room occupancy slot;
- workstation;
- record page under writing;
- institutional staff time;
- funds held for payment;
- route segment traversal if congestion matters.

Conflicts produce rejection, waiting, contention, interruption, or a race. The choice is action-defined and scheduler-mediated.

## Scheduling and interruption

Some actions resolve immediately. Many actions take simulated time.

Scheduled actions must define:

- start event policy;
- duration model;
- interruptibility windows;
- reservation lifetime;
- observation opportunities during execution;
- partial progress;
- failure events;
- cleanup/release rules.

Example:

```text
StartOpenStrongbox
 -> duration begins
 -> nearby actor may hear metal noise
 -> owner may enter room and interrupt
 -> lock may resist
 -> action resolves as opened, failed, damaged, or abandoned
```

Schedules are not teleports. A work routine, trip to office, sleep action, or theft attempt is a chain of scheduled and observable actions.

## Outcome resolution

Outcome resolution uses authoritative state, actor state, action definition, relevant beliefs, reservations, environment, norms, and controlled randomness.

It must produce one of:

- committed success event;
- committed failure event;
- no-op rejection with why-not;
- scheduled continuation;
- interruption event;
- escalation to another action or process.

Outcome resolution must not use dramatic need, player importance, hidden quest state, LLM prose, or convenience truth.

## Traces and observations

Action definitions specify possible traces and observation hooks. The pipeline decides which traces occur and who observes them.

A trace is not a clue created for the player. It is a consequence of the action.

A witness does not receive truth. A witness receives observations through channels:

```text
noise heard -> interpreted as possible movement
shape seen in low light -> interpreted as someone similar to Mara
empty strongbox observed -> interpreted as missing coins if expected coins were there
ledger page altered -> interpreted as possible mistake, correction, or forgery depending on observer knowledge
```

## Projection and reaction updates

After event commit, projections update:

- current state;
- actor beliefs and memories;
- trace indexes;
- institutional records;
- lead cards;
- TUI view models;
- debug causal graph;
- story-sifter observer summaries.

Projection updates are derived. They must be rebuildable from events and content/data versions.

## TUI affordance menus

The embodied TUI shows actor-filtered affordances. It does not show hidden truth or impossible actions unless presented as explicit unknown attempts.

Acceptable menu entries:

```text
Open strongbox
Search strongbox
Ask Elena about the noise
Report missing coins to clerk
Accuse Mara based on suspicion
Wait until office opens
```

Unacceptable menu entries:

```text
Recover stolen coins from Mara's mattress
Complete theft quest
Reveal culprit
Fast travel to actual culprit
Use player-only inspect truth
```

Debug mode may show truth-only actions, but they must be visibly non-diegetic and must not mutate ordinary world state without explicit debug-event handling.

## Why-not explanations

Rejected proposals produce why-not explanations from the same checks used for validation.

Example:

```yaml
WhyNot:
  action: report_missing_item
  actor: actor_tomas
  result: unavailable
  reasons:
    - clerk_office_closed_until_morning
    - actor_does_not_know_any_authority_receiver_elsewhere
  possible_actor_actions:
    - wait
    - go_home
    - ask_guard_if_visible
    - write_private_note
```

Why-not explanations must be actor-filtered in embodied mode. Debug mode may include hidden validation details.

## Acceptance implications

A feature touching world state must ship with tests for:

- human and AI proposal parity;
- rejection without mutation;
- failure event commit when meaningful;
- trace creation;
- observation filtering;
- belief/projection update;
- replay determinism;
- no-human execution;
- TUI affordance visibility;
- debug explanation.

## Anti-patterns

- UI button mutates inventory.
- Content script awards item directly.
- Player-only `take_all` action.
- NPC can report only if player has accepted a quest.
- Door opens because scene transition needs it.
- Schedule places actor at work without travel or belief.
- LLM dialogue invents a clue and commits it.
- Institution records a case without receiver, procedure, and physical or digital record artifact.
- Trace appears because the player needs a lead.
