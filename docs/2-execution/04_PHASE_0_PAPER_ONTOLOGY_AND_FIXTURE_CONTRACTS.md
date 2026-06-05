# Phase 0: Paper Ontology and Fixture Contracts

## Purpose

Phase 0 is a design proof, not an executable milestone.

It exists to prevent the first implementation from smuggling in quests, hidden scripts, omniscient records, player privilege, or LLM authority. The team must manually trace the missing-property village from cause to consequence before writing the runnable kernel.

Phase 0 answers:

```text
Can the first proof be described as ordinary actions, events, traces,
observations, beliefs, reports, records, suspicion, replay, and debug views?
```

If the answer requires a quest object, plot flag, player identity, hidden culprit script, or LLM reasoning, Phase 0 fails.

## Entry requirements

Phase 0 may begin when:

- the execution charter is accepted;
- The Missing Property Village is accepted as the first proof;
- road-threat/travel/companion systems are marked second-proof;
- the current foundation and architecture authority set is identified;
- the uploaded manifest has been used as inventory;
- the old execution docs are treated as source material, not final truth.

## Deliverables

### Primitive action vocabulary

Define about 20 primitive action families. These are vocabulary contracts, not implementation code.

Required first list:

```text
Look
MoveToAdjacentPlace
OpenDoor
CloseDoor
LockDoor
UnlockDoor
InspectObject
OpenContainer
CloseContainer
SearchContainer
SearchPlace
TakeItem
PlaceItem
HideItem
Eat
Sleep
Wait
BeginWorkBlock
ContinueRoutine
InterruptRoutine
SpeakStructuredAct
Ask
Answer
Tell
Report
Gossip
Refuse
Lie
AccuseOrSuspect
ReadRecordOrNotice
WriteRecordEntry
```

This list may be trimmed or split, but the first proof must support movement, door/container use, item handling, sleep/eat/work/wait, structured speech, report, and record writing.

Each action card must include:

- actor requirements;
- physical preconditions;
- knowledge requirements;
- social/normative checks;
- cost/duration;
- reservations if any;
- possible outcomes;
- traces;
- observation hooks;
- norm hooks;
- event kinds;
- TUI label;
- why-not reason classes;
- failure modes.

### Primitive event vocabulary

Define event kinds for the first proof.

Required first list:

```text
ActorMoved
DoorOpened
DoorClosed
ContainerOpened
ContainerClosed
ItemRemovedFromContainer
ItemPlacedInContainer
ItemHidden
ItemPossessionChanged
ActionRejected
ActionStarted
ActionInterrupted
ActionFailed
ObservationCreated
BeliefUpdated
ExpectationContradicted
TraceCreated
TraceAltered
SpeechActCommitted
ReportReceived
LedgerEntryCreated
RecordRead
RoutineStarted
RoutineInterrupted
NeedChanged
SleepStarted
SleepCompleted
WorkBlockStarted
WorkBlockCompleted
ControllerAttached
ReplayCheckpointCreated
```

Meaningful changes require event ancestry. Current-state mutation without event is forbidden for proof-relevant changes.

### Primitive proposition vocabulary

Define proposition shapes that beliefs and records may hold.

Required first list:

```text
ItemLocatedInContainer(item, container)
ItemMissing(item)
ItemLastExpectedAt(item, place_or_container)
ActorAtPlace(actor, place)
ActorNearPlace(actor, place, time_range)
ContainerOpenedRecently(container)
SoundHeardNear(place, time_range)
ActorHadAccess(actor, place_or_container)
ActorHadMotive(actor, motive_claim)
ActorOwnsItem(actor, item)
ActorPossessesItem(actor, item)
ActorPermittedToUse(actor, object)
ReportClaimsMissingItem(report, item)
InstitutionOpenedRecord(institution, record)
ActorSuspectedBy(actor_or_institution, suspect, reason)
RecordContainsClaim(record, proposition)
```

Propositions are claims, not truth flags. They need holder, source, confidence, and acquisition time when believed.

### Primitive trace vocabulary

Define trace types.

Required first list:

```text
absence_marker
disturbed_container
open_or_closed_state_anomaly
lock_scratch
noise_trace
partial_glimpse
moved_object
footprint_or_mud_if_supported
record_entry
rumor_packet
erased_trace
nervous_behavior_observation
```

Traces must define possible interpretations. Avoid perfect clues.

### Primitive speech-act vocabulary

First proof speech acts:

```text
greet
ask
answer
tell
report
gossip
refuse
lie
accuse_or_suspect
testify_or_statement
withhold_or_deflect
promise_if_needed
command_or_instruct_if_role_permits
```

Each speech act must specify:

- speaker;
- listeners;
- propositions;
- source beliefs;
- asserted confidence;
- whether truthful, lie, speculation, report, hearsay, or question;
- validation checks;
- listener interpretation;
- possible refusal/failure.

### Actor roster

Create first-proof actor cards for:

- Tomas;
- Mara;
- Elena;
- Anna;
- Elias;
- Reeve;
- 4–10 supporting locals.

Each card must include:

```text
home
work or survival strategy
household
needs profile
durable concern/project
relationships
initial beliefs with sources
routine sketch
property/custody/access facts
possible role in wrong suspicion
```

No actor exists solely to serve the player.

### Places and object inventory

Define:

- homes and rooms;
- doors and access;
- containers and locks;
- beds;
- food storage;
- workplace;
- local authority office;
- incident ledger artifact;
- simple money/custody object;
- optional public food/social place.

Each object must expose affordances, not outcomes.

### Required manual chains

Deliver paper traces for:

1. strongbox/missing-property chain;
2. ordinary workday chain;
3. report/record chain;
4. wrong suspicion chain;
5. possession parity chain;
6. no-human daily chain.

Each chain must include events, cause links, observations, beliefs, records, debug visibility, TUI visibility, and failure cases.

## TUI screen sketches

Phase 0 must sketch these screens as view-model contracts:

### Embodied local view

Shows:

- current actor;
- actor-known place;
- visible/perceived entities;
- known exits;
- actor-known status;
- actor-filtered action menu;
- why-not request path;
- actor-known notebook excerpt;
- actor-perceived recent events.

Must not show:

- hidden culprit;
- true item location;
- other actors' private beliefs;
- debug event log;
- hidden traces.

### Object/container view

Shows:

- actor-known object identity;
- perceived state;
- visible contents if opened/searched and perceivable;
- actor-available affordances;
- actor-known ownership/privacy risk;
- why-not reasons.

### Actor-known notebook

Shows:

- source-bound beliefs;
- confidence/uncertainty;
- last verification;
- contradictions;
- conversations heard;
- reports made/read;
- actor-known leads.

### Structured speech/report view

Shows:

- possible speech acts;
- target listener;
- source beliefs used;
- uncertainty;
- social risk;
- validation failures.

### Debug event/causal view

Shows:

- ground truth;
- event log;
- causal graph;
- projection diffs;
- hidden beliefs;
- hidden traces;
- planner traces;
- possession history;
- replay diagnostics.

Debug must be visibly non-diegetic.

## View-model contracts

Every view-model sketch must specify:

```text
input projections
knowledge context
allowed truth source
forbidden truth source
stable semantic action IDs
why-not source
test assertions
debug counterpart
```

Bad contract:

```text
TUI queries world state and hides fields manually.
```

Good contract:

```text
EmbodiedViewModel(actor_tomas) is produced by actor-knowledge projection
and contains no unobserved event-log truth.
```

## No-scripting review

Review all Phase 0 chains against these forbidden patterns:

- `Quest`;
- `PlayerCharacter`;
- `ObjectiveMarker.true_location`;
- `NPC.knows_truth`;
- `scripted culprit`;
- `scripted accusation`;
- `scripted report acceptance`;
- `Reward.spawn_on_completion`;
- `LLM_mutated_state`;
- direct content inventory mutation;
- guaranteed outcome sequence;
- NPC waiting for player;
- institution reading true culprit;
- debug knowledge as actor knowledge.

If any appear, rewrite the fixture as causal possibility.

## TUI/view-model gate

Phase 0 passes the TUI gate only if a reviewer can follow the first proof through proposed embodied and debug screens and see exactly where hidden truth is excluded.

The gate must include paper assertions for:

- Tomas before checking strongbox;
- Tomas after checking strongbox;
- Mara after taking/hiding item;
- Elena after hearing uncertain sound;
- Anna after receiving report;
- Elias after reading record;
- debug observer after all events.

## No-human simulation gate

The paper no-human chain must show:

- scheduler decision points;
- routines proceeding;
- Mara's motive/opportunity emerging without human input;
- Tomas discovering or failing to discover absence;
- report/record path if conditions warrant;
- no event referencing player identity.

If the chain needs a human click to begin, Phase 0 fails.

## Deterministic replay gate

Each chain must list replay-critical data:

- event order;
- stable IDs;
- content version;
- random streams if any;
- meaningful random draw purposes;
- projection rebuild targets;
- debug explanation queries.

## Test gate

Define at least these golden scenario sketches:

- `strongbox_001`;
- `expectation_contradiction_001`;
- `possession_parity_001`;
- `report_record_001`;
- `wrong_suspicion_001`;
- `no_human_day_001`;
- `view_filtering_001`;
- `replay_rebuild_001`.

Each sketch must include expected occurrence and expected non-occurrence.

## Data/fixture gate

The paper fixture must specify:

- stable IDs;
- actor roster;
- place graph;
- object inventory;
- initial beliefs with provenance;
- expectations;
- needs/pressures;
- routine skeletons;
- relationships;
- household memberships;
- institution roles;
- norms;
- record schemas;
- scenario seed constraints;
- no forced outcomes.

## Debug/inspection gate

For each manual chain, define debug answers to:

```text
What happened?
Why was it possible?
What action pipeline checks passed or failed?
What traces exist?
Who observed what?
Who believes what?
Which beliefs are wrong?
What record claims exist?
What institution knows what?
What later events became possible?
What would replay rebuild?
```

## Forbidden shortcuts

- finalizing crates before ontology;
- writing Rust code before action/event vocabulary;
- choosing data syntax before validation contracts;
- designing a notice-board UI as first proof;
- treating "The Village That Notices" as road-threat play;
- authoring a culprit sequence;
- using LLM text as social simulation;
- accepting fixture prose that cannot become events.

## Exit checklist

Phase 0 exits only when all are true:

- first-proof chain manually traces from cause to consequence;
- action/event/proposition/trace/speech-act vocabularies exist;
- first village actor roster exists;
- place/object/household/institution inventory exists;
- view-model contracts exist;
- TUI sketches separate embodied and debug truth;
- golden scenario sketches exist;
- no-scripting review passes;
- no-human chain exists;
- replay-critical data is identified;
- deferrals are explicit.

Do not proceed to Phase 1 without this checklist.
