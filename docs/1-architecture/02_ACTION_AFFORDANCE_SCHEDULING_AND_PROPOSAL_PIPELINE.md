# Action, Affordance, Scheduling, and Proposal Pipeline

## Status

This document defines the architecture contract for every world-affecting action in Tracewake.

The same pipeline is used by human-controlled actors, autonomous agents, institutions, routines, and non-agent processes. There are no player-only world verbs.

## Core rule

A world-affecting change is allowed only when a valid actor or process submits a proposal that passes the action pipeline, is scheduled/resolved as required, and commits events.

It is not allowed because:

- a UI button requested it;
- a content file scripted it;
- a quest state needs it;
- an LLM described it;
- a debug tool was convenient;
- the player is important.

## Authority

This subsystem owns:

- command/proposal interpretation;
- action definition lookup;
- affordance binding;
- physical, epistemic, social, resource, and reservation checks;
- scheduling and interruption integration;
- outcome-resolution contract;
- rejection/failure distinctions;
- trace and observation hooks;
- norm and institution hooks;
- why-not reports.

It is denied:

- hidden dramatic overrides;
- player privilege;
- direct UI mutation;
- ground-truth leakage into actor knowledge;
- LLM authority;
- event-log bypass.

## Terms

### Command

A command is a boundary input or request.

```yaml
Command:
  source: human_controller
  bound_actor: actor_tomas
  intent: search_container
  target: strongbox_tomas
```

Commands are not world facts. They become proposals or rejections.

### Proposal

A proposal is a fully parameterized candidate action from a command, planner, routine, institution procedure, or process.

```yaml
ActionProposal:
  proposer: actor_mara
  action: remove_item_from_container
  parameters:
    container: strongbox_tomas
    item: coin_stack_01
  basis:
    - belief_mara_tomas_has_coins
    - project_repay_debt
```

### Action definition

An action definition is a rule for a kind of possible world transition. It defines parameters, preconditions, duration, reservations, costs, failure modes, outcome semantics, trace hooks, observation hooks, norm hooks, and event kinds.

### Affordance

An affordance is a typed capability exposed by an entity, place, role, record, social context, or process.

Examples:

```text
door: open, close, lock, unlock, force, listen_through
bed: sleep, nap, inspect_occupied
container: open, close, inspect_contents, place_item, remove_item, search
record: read, write_entry, amend, copy, forge, destroy
notice_board: read_notice, post_notice, remove_notice, deface_notice
person: greet, ask, answer, report, lie, refuse, accuse, promise, testify
road: travel, wait, inspect_tracks, ask_about_route
workplace: begin_work, pause_work, close_shop, sell_item, record_debt
```

Affordances produce candidate actions. They never bypass validation.

### Rejection versus failure

A rejection means the action never began because a required condition was not satisfied. Rejections may be displayed as why-not explanations. They become events only when future reasoning can care about the failed attempt.

A failure means the action began or became socially/causally meaningful, then did not achieve its intended outcome. Failures are event-worthy when visible, remembered, trace-producing, or causally relevant.

## Unified pipeline

Every world-affecting proposal passes through this conceptual pipeline:

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
12. immediate resolution or scheduled-action creation
13. interruption-window checks while running
14. outcome resolution
15. state mutation plan
16. trace creation plan
17. observation pass
18. belief/projection/reaction hooks
19. institution/norm hooks
20. event commit
21. projection/view-model update
22. follow-up scheduling
```

Simple actions may make some steps vacuous. No client may skip the pipeline.

## Action definition contract

Pseudo-structure:

```yaml
ActionDefinition:
  id: remove_item_from_container
  category: ordinary_life
  parameters:
    container: EntityRef<Container>
    item: EntityRef<Item>
  afforded_by:
    entity_components: [Container]
  actor_requirements:
    body_capabilities: [can_reach, can_carry]
    agency_state: conscious_or_automated_process
  preconditions:
    physical:
      - actor_can_reach_container
      - item_present_or_action_is_search
      - actor_can_carry_item
    knowledge:
      - actor_perceives_item OR actor_has_search_basis
      - target_location_known_or_discoverable
    social_normative:
      - actor_has_permission OR actor_willing_to_violate_property_norm
    resource:
      - actor_has_time_or_accepts_risk
  reservations:
    - actor_body
    - container_access
    - item_possession_candidate
  duration:
    base: short
    modifiers: [lock_state, lighting, stealth, fatigue]
  interruption_windows:
    - witness_entry
    - actor_abort
    - container_state_changed
  possible_outcomes:
    - item_removed
    - item_not_found
    - interrupted
    - caught_in_act
    - container_damaged
  traces:
    - disturbed_container
    - possible_noise
    - absence_trace_if_expected
  observation_hooks:
    sight: local_visibility
    sound: room_sound_propagation
  norm_hooks:
    - property_violation_candidate
    - trespass_violation_candidate
  events:
    start: ActionStarted
    success: ItemRemovedFromContainer
    failure: ActionFailed
```

This is contractual data shape, not implementation code.

## Actor and process authority

An actor can propose an action only if it has world authority to attempt it:

- body capability;
- agency/consciousness state;
- physical access;
- possession/control binding if command came from a human;
- role permission where relevant;
- willingness to violate norms when permission is absent;
- knowledge or search basis sufficient to attempt;
- available time/resources;
- process authority for non-agent processes such as fire, disease, weather, or regional boundary events.

Human control grants input authority only. It never grants world authority.

## Knowledge preconditions

Actors act from beliefs, perception, records, memory, testimony, inference, speculation, and search intentions. Knowledge checks are not truth checks.

Good:

```text
Tomas can accuse Mara if Tomas has actor-known basis, such as hearing Mara was nearby,
knowing Mara has debt pressure, receiving a report, or choosing a reckless accusation.
```

Bad:

```text
Tomas can accuse Mara because debug truth says Mara stole the coins.
```

Search actions are first-class because they allow actors to discover whether a belief is true without reading hidden state.

## Social and normative checks

Norms do not simply block actions. They shape permission, obligation, deterrence, willingness to violate, observation, reportability, and later consequences.

```yaml
SocialNormativeCheck:
  norm: household_privacy
  action: enter_private_room
  permitted: false
  attempt_allowed_if_willing_to_violate: true
  visible_violation_if_observed: trespass_candidate
  likely_reactions: [confront, remember, report]
```

The architecture must preserve this sequence:

```text
violation -> detection -> suspicion -> report -> record -> proof -> sanction
```

No stage implies the next one automatically.

## Scheduling and duration

Actions may be near-instant, short, extended, or interval-based. Durations are simulation facts because they create opportunities for observation, interruption, conflict, and missed expectations.

Scheduled action instances must define:

- actor/process;
- start tick;
- earliest/latest completion;
- reservations;
- interruptibility windows;
- observation windows;
- partial-progress policy;
- failure/cleanup/release policy;
- event policy for start, progress, completion, interruption, and cancellation.

```yaml
ScheduledAction:
  id: sched_000882
  action: remove_item_from_container
  actor: actor_mara
  start_tick: 81210
  earliest_completion_tick: 81230
  latest_completion_tick: 81245
  reservations:
    - actor_body: actor_mara
    - container_access: strongbox_tomas
  observation_windows:
    - kind: possible_sound
      tick_range: [81215, 81220]
  interrupt_windows:
    - visible_witness_entry
    - actor_choice_abort
    - danger
  status: running
```

Schedules are not teleports. Work, travel, sleep, report writing, theft, and questioning are observable chains or intervals.

## Reservations and conflicts

Reservations prevent impossible overlap and model contention.

Reservation categories include:

- actor body/action channel;
- hands/carry capacity;
- item possession;
- container access;
- door operation;
- bed occupancy;
- room/path occupancy where needed;
- workplace station;
- record-writing surface;
- institutional staff time;
- funds/custody commitment;
- route segment if congestion/risk matters.

Conflicts produce legible outcomes: wait, reject, race, interrupt, negotiate, shove, steal, or fail. The action definition and scheduler decide; runtime accident must not.

## Stable ordering and randomness touchpoint

Same-tick proposals and completions require deterministic ordering. Ordering must not depend on wall-clock time, hash-map iteration accident, UI frame order, thread scheduling, network latency, or live LLM response timing.

Meaningful random branches are consumed through scoped, purpose-labeled random streams owned by the replay/save-package contract.

## Outcome resolution

Outcome resolution uses authoritative state, actor state, relevant beliefs, action definition, reservations, environment, norms, and controlled randomness.

It produces one of:

- committed success event;
- committed failure event;
- no-op rejection with why-not;
- scheduled continuation;
- interruption event;
- escalation to another proposal/process.

It must not use dramatic need, hidden quest state, player importance, LLM prose, or convenience truth.

## Traces and observations

Action definitions specify possible traces and observation hooks. The pipeline decides which traces occur and who observes them.

A trace is not a clue created for the player. It is a consequence.

A witness does not receive truth. A witness receives channel-specific observations:

```text
noise heard -> possible movement near room
shape in low light -> person similar to Mara, not confirmed Mara
empty strongbox -> absence only for actor with expectation
altered ledger page -> possible mistake, correction, or forgery depending on observer knowledge
```

## TUI affordance menus and why-not

Embodied TUI menus are generated from actor-visible affordances and actor-known possibilities.

Acceptable entries:

```text
Search strongbox
Ask Elena what she heard
Report missing coins to clerk
Wait until office opens
Accuse someone based on suspicion or reckless claim
```

Unacceptable entries:

```text
Recover stolen coins from Mara's mattress
Complete theft quest
Reveal culprit
Use player-only inspect truth
```

Why-not reports come from the same validation checks and are filtered by view mode.

```yaml
WhyNot:
  action: report_missing_item
  actor: actor_tomas
  result: unavailable
  embodied_reasons:
    - office_closed
    - no_known_report_receiver_present
  possible_actor_actions:
    - wait
    - ask_visible_guard
    - write_private_note_if_materials_available
  debug_reasons:
    - clerk_asleep_at_home
    - guard_on_south_patrol_route
```

## Acceptance implications

A world-affecting feature must test:

- human and AI proposal parity;
- rejection without mutation;
- failure event commit when meaningful;
- duration/reservation/interruption behavior;
- trace creation and observation filtering;
- belief/projection update;
- norm/institution hook behavior;
- no-human execution;
- deterministic replay;
- TUI affordance visibility;
- embodied why-not filtering;
- debug explanation.

## Anti-patterns

- UI button mutates inventory.
- Content script awards or removes an item directly.
- Player-only `take_all` action.
- NPC reports only after player accepts a quest.
- Door opens because a scene transition needs it.
- Schedule places actor at work without travel or belief.
- Trace appears because the player needs a lead.
- LLM dialogue invents and commits a clue.
- Institution record appears without receiver, writer, procedure, and artifact.
