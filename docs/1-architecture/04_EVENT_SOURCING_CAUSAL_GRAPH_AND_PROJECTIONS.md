# Event Sourcing, Causal Graph, and Projections

## Status

This document defines Tracewake's event-sourcing architecture contract.

Tracewake avoids current-state-only simulation. Current state is a projection of event history plus content/data versions plus optional snapshots. Meaningful world mutation commits through events.

## Why event sourcing is foundational

Tracewake requires event sourcing because it must answer:

- What happened?
- What caused it?
- Who observed it?
- Who believed it?
- Which records, rumors, traces, and opportunities came from it?
- Why was an action possible?
- Why was an action rejected or failed?
- Did the player and NPCs use the same pipeline?
- Can no-human simulation be inspected?
- Can projections be rebuilt deterministically?
- Can a stale notice, wrong suspicion, or institutional error be traced to its source?

A conventional mutable save file is insufficient.

## Event categories

Events may represent:

- successful world mutation;
- attempted action;
- failed action;
- observation;
- belief update;
- memory change;
- trace creation, alteration, or erasure;
- speech act;
- rumor propagation;
- institutional report, record, decision, notice, order, sanction, or failure;
- ownership/custody transfer;
- schedule start, completion, interruption, or cancellation;
- LOD promotion/demotion;
- summary of low-salience activity;
- exogenous regional/boundary process;
- debug/controller binding metadata when relevant to replay but not world truth.

Not every microscopic calculation is an event. Every causally meaningful mutation is.

## Event envelope

Every event uses a versioned envelope.

```yaml
EventEnvelope:
  event_id: evt_000018239
  event_type: item_removed_from_container
  event_schema_version: 2
  stream_positions:
    world: 18239
    entity_item_coin_stack_01: 77
    actor_mara: 391
    place_tomas_bedroom: 51
  sim_time:
    tick: 81230
    calendar: 142-08-12T02:13
  ordering:
    scheduler_sequence: 591233
    tie_breaker: deterministic_actor_order
  actor: actor_mara
  process: null
  place: room_tomas_bedroom
  participants:
    container: strongbox_tomas
    item: coin_stack_01
  causes:
    - kind: intention
      id: intention_mara_take_coins
    - kind: belief
      id: belief_mara_tomas_has_coins
    - kind: need_pressure
      id: need_mara_debt_pressure
    - kind: enabling_event
      id: evt_000018232
  random_draws:
    - stream: stealth_noise
      draw_id: rnd_8821
      purpose: noise_trace_resolution
      value_commitment: recorded_or_hash_per_build_policy
  payload:
    kind_specific_fields: here
  effects_summary:
    - coin_stack_01 possession changed from strongbox_tomas to actor_mara
  traces_created:
    - trace_disturbed_container_44
  observations_created:
    - obs_elena_heard_noise_12
  projection_hints:
    - ownership_index
    - place_trace_index
    - actor_belief_candidate_index
  content_versions:
    ruleset: tracewake-core-rules@0.1
    domain_pack: neutral_village@0.1
  debug:
    validation_report_id: val_9321
    replay_checksum_after_event: optional
```

The payload is typed by event kind. The envelope supplies ordering, causality, versioning, replay, and projection metadata.

## Event kind and payload

Event kind identifies semantics. Payload carries event-specific data.

Rules:

- Event kinds are stable semantic contracts.
- Payload schemas are versioned.
- Events are immutable after commit.
- Corrections are new events, not edits.
- Old events remain readable through version handlers or upcasters.
- Events should be named after what occurred, not after UI intentions.

Good:

```text
ItemRemovedFromContainer
ReportReceived
BeliefUpdatedFromSpeech
NoticePosted
ActionInterrupted
TraceErased
LODPromoted
SummaryRoutinePeriodCommitted
```

Bad:

```text
QuestStep2Complete
PlayerClickedButton
SpawnBanditsForDrama
GiveReward
```

## Event ordering

Tracewake uses discrete simulation time plus deterministic ordering.

Ordering fields must support:

- global order for replay;
- stream-local order for efficient lookup;
- stable tie-breaking for same-tick events;
- replay checksum or projection version comparison;
- deterministic handling of simultaneously scheduled actions.

Rules:

- No wall-clock time determines event order.
- Unordered collection iteration must not determine event order.
- Concurrent implementation may exist later, but commit order must remain deterministic.
- Same-tick conflicts are resolved by explicit scheduler policy, not runtime accident.

## Streams and indexes

The primary source is a globally ordered world event stream. Secondary streams/indexes support inspection and performance.

```text
WorldStream                  all committed significant events in global order
EntityStream(entity_id)      events involving an entity
ActorStream(actor_id)        caused, perceived, believed, intended, spoken events
PlaceStream(place_id)        events and traces at a place
InstitutionStream(inst_id)   reports, records, procedures, notices, sanctions, failures
RecordStream(record_id)      ledger/case/notice lifecycle
ItemStream(item_id)          ownership, possession, custody, movement, damage
RouteStream(route_id)        travel, patrols, risk, rumors, boundary events
RumorStream(topic_id)        rumor propagation, mutation, contradiction, source loss
ControllerStream(controller) possession/debug binding history, not world truth
ProjectionStream(name)       rebuild/version/checkpoint metadata
```

Secondary streams may be stored, indexed, or rebuilt. They must not contradict the world stream.

## Causality links

Causal links are typed edges.

```yaml
CausalLink:
  from: evt_000018232
  to: evt_000018239
  relation: enabling_condition
  explanation: door was opened before container access
```

Allowed relation categories:

- direct physical cause;
- enabling condition;
- actor intention;
- need pressure;
- value or motive;
- belief or memory;
- observation;
- speech act;
- rumor;
- institutional procedure;
- role obligation;
- norm implication;
- relationship motive;
- resource constraint;
- scheduled continuation;
- interruption;
- random branch;
- environmental process;
- regional process;
- LOD summary ancestry;
- debug/controller metadata when strictly outside world truth.

Causality links are not prose annotations only. They must be queryable enough for debug inspection and acceptance tests.

## Causal graph inspection

Debug tools must be able to inspect:

- upstream causes for an event;
- downstream consequences from an event;
- why an action was possible;
- why an action failed;
- which traces came from an event;
- who observed an event or trace;
- who formed beliefs from it;
- which institutional records cite it;
- where a wrong belief originated;
- which summary event hides lower-detail ancestry;
- which random draw affected a meaningful branch.

Canonical debug question:

```text
Why does Tomas suspect Mara?
```

Expected answer shape:

```text
Tomas found strongbox empty.
He expected coins there from prior direct observation.
Elena reported hearing a noise near the room at night.
Tomas knows Mara had debt pressure from prior gossip.
No one saw the theft directly.
Suspicion confidence is medium and may be wrong.
```

## Projection types

Projections are derived, rebuildable read models.

Required projection families:

- current component state;
- action-affordance indexes;
- actor belief stores;
- actor memory summaries;
- trace indexes;
- observation indexes;
- institution records;
- notice board state;
- ownership/custody indexes;
- route risk/public belief indexes;
- lead cards;
- TUI embodied view models;
- debug view models;
- causal graph indexes;
- story-sifter observer summaries;
- replay checksums and migration state.

Projection rebuild must be deterministic for a given event log, content version set, and projection version.

## Event-derived beliefs and records

Beliefs and records may be stored as projections or as explicit events depending on causal significance.

Architecture rule:

- If a belief/record change can affect future decisions, testimony, institutional procedure, or debug explanation, it must have event-level provenance.
- High-volume low-salience memory maintenance may be summarized, but source preservation is required for important beliefs.
- Institutional records are world artifacts. Their lifecycle events must be committed.

Example:

```text
SpeechActCommitted
 -> ListenerObservedSpeech
 -> BeliefUpdatedFromSpeech
 -> RumorSeedCreated
```

The belief projection may cache the current belief. The event log must explain how it got there.

## Snapshots

Snapshots improve replay performance. They are not replacements for causal ancestry.

Snapshot contents may include:

- current component state;
- scheduler queue;
- random stream positions;
- projection checkpoints;
- content/data manifest;
- LOD tier state;
- compaction references;
- checksum metadata.

Snapshot rules:

- A snapshot must declare the exact event position it covers.
- A snapshot must declare content/data versions and projection versions.
- A snapshot must not erase event ancestry needed by traces, beliefs, records, active leads, investigations, wrong suspicions, or story sifting.
- A snapshot may be discarded and rebuilt from events.

## Compaction and summary events

Long simulation requires compaction. Compaction creates summary events, not hidden edits.

Preserve detailed events for:

- player/NPC state-changing actions;
- ownership, possession, and custody changes;
- surviving traces;
- belief-changing events;
- contradiction events;
- institutional records and procedures;
- reports, testimony, notices, contracts, sanctions;
- crimes and suspected crimes;
- injuries/deaths;
- active leads;
- events referenced by records, speech, or memory;
- LOD promotion/demotion causes;
- random branches that materially changed outcomes.

Summarize:

- repeated routine meals;
- uneventful work spans;
- low-salience background travel;
- stable household maintenance;
- regional process aggregates.

A routine event becomes non-routine if it later matters. The architecture must support retaining or promoting ancestry when a previously ordinary event becomes evidence.

## Event schema versioning and upcasters

Event schema evolution is mandatory. Stored events remain immutable.

Policy:

- Every event has a schema version.
- Non-breaking additions may be handled by defaults.
- Breaking changes require new event types, version handlers, or upcasters.
- Upcasters transform old event representations at read/replay time without mutating stored events.
- Upcaster chains are tested with historical fixtures.
- Projection versions declare which event versions they support.
- Replay rejects unknown event versions unless a migration/upcaster path exists.

Pseudo-registry:

```yaml
EventSchemaRegistry:
  event_type: item_removed_from_container
  supported_versions: [1, 2]
  current_version: 2
  upcasters:
    - from: 1
      to: 2
      rule: add_reservation_context_from_envelope_default
  replay_policy: reject_without_upcaster
```

## Replay failure handling

Replay failure is a first-class diagnostic, not a silent repair.

Failures include:

- unknown event type;
- unsupported event schema version;
- missing content/data version;
- projection checksum mismatch;
- nondeterministic ordering;
- random stream mismatch;
- upcaster failure;
- invariant violation;
- projection drift;
- missing causal reference.

Failure response:

```text
stop replay
record replay failure report
identify event position and projection
show content/data versions
show expected vs actual checksum where available
require migration/upcaster/data fix before accepting save/log
```

Do not continue replay with best-effort guesses in authoritative mode.

## Random draw recording

Meaningful random branches must be auditable.

Record at least:

- random stream identity;
- draw purpose;
- draw sequence or commitment;
- event or decision that consumed it;
- result category if needed for explanation.

Debug builds may record raw values. Release builds may record enough commitment/checksum data to verify replay policy. Acceptance tests should be able to assert deterministic branches.

## Event-derived view models

The TUI and future clients consume view models derived from events and actor-filtered projections.

Embodied view model rule:

```text
No embodied view model reads ground truth unless the current actor could know it through modeled channels.
```

Debug view model rule:

```text
Debug may read truth, event logs, causal graph, hidden beliefs, and projection diffs, but must be visibly non-diegetic.
```

## Anti-patterns

- Saving only current mutable state.
- Editing old events to repair history.
- Quest completion flags as truth.
- Projection cache treated as authority.
- Event without cause for a meaningful mutation.
- Belief without source.
- Notice without author, issuer, claims, and posting event.
- Random outcome without seed discipline or audit policy.
- Snapshot that erases active causal ancestry.
- Replay that silently skips unknown events.
