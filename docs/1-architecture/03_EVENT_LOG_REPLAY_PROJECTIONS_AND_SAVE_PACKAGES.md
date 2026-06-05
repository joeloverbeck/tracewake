# Event Log, Replay, Projections, and Save Packages

## Status

This document defines the architecture contract for event sourcing, causal graph inspection, deterministic replay, projections, snapshots, event schema evolution, random streams, and save packages.

Tracewake avoids current-state-only saves. The authoritative answer to “what happened?” lives in the event log plus content/schema versions. Current state is a projection.

## Core rule

Meaningful world mutation commits through immutable, versioned events. Projection caches, snapshots, UI view models, lead cards, and metrics are derived. If they disagree with replay from the event log and declared content versions, the derived artifact is wrong or the replay must fail loudly.

## Authority

This subsystem owns:

- append-only event semantics;
- event envelopes and schema versions;
- global and stream-local ordering;
- causal graph links;
- projection rebuild contracts;
- snapshot contracts;
- save-package manifests;
- content/schema/upcaster compatibility;
- deterministic random stream state;
- replay modes and failure reports;
- compaction and summary ancestry policy.

It is denied:

- editing old events;
- silently repairing history;
- treating snapshots as causal truth;
- accepting unknown versions by guess;
- allowing storage adapters to define domain rules;
- hiding active causal ancestry behind compaction.

## Event categories

Events may represent:

- successful world mutation;
- meaningful attempted or failed action;
- scheduled action start/progress/completion/interruption/cancellation;
- observation;
- belief update;
- memory change;
- trace creation, alteration, decay, discovery, or erasure;
- speech act;
- rumor propagation/mutation;
- report, record, notice, institutional decision, sanction, or procedure failure;
- ownership, possession, custody, debt, or promise change;
- LOD promotion/demotion;
- summary of low-salience activity;
- regional/boundary process;
- debug/controller metadata when needed for replay but not world truth.

Not every calculation is an event. Every causally meaningful mutation is.

## Event envelope

Every event uses a versioned envelope.

```yaml
EventEnvelope:
  event_id: evt_000018239
  event_type: item_removed_from_container
  event_schema_version: 2
  stream_positions:
    world: 18239
    actor_mara: 391
    item_coin_stack_01: 77
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
      id: intention_mara_find_money_quietly
    - kind: belief
      id: belief_mara_tomas_has_coins
    - kind: need_pressure
      id: need_mara_debt_pressure
    - kind: enabling_event
      id: evt_door_left_unlocked
  random_draws:
    - stream: trace_generation
      draw_id: rnd_8821
      purpose: possible_noise_trace
      result_category: low_noise
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

The payload is typed by event kind. The envelope carries ordering, causality, versioning, replay, and projection metadata.

## Event naming

Good event names describe what occurred:

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

Bad event names describe UI, quests, or drama:

```text
QuestStep2Complete
PlayerClickedButton
SpawnBanditsForDrama
GiveReward
```

## Ordering

Tracewake uses discrete simulation time plus deterministic ordering.

Ordering must support:

- global replay;
- stream-local lookup;
- stable same-tick tie-breaking;
- deterministic reservation conflict handling;
- random draw sequence integrity;
- replay checksum/projection comparison.

Forbidden ordering sources:

- wall-clock time;
- OS thread scheduling;
- hash-map iteration accident;
- terminal/graphical frame order;
- network latency;
- live LLM response timing.

## Streams and indexes

The world stream is primary. Secondary streams may be stored or rebuilt.

```text
WorldStream                  all committed significant events
ActorStream(actor_id)        caused, perceived, believed, intended, spoken events
EntityStream(entity_id)      events involving an entity
PlaceStream(place_id)        events/traces at a place
InstitutionStream(inst_id)   reports, records, notices, sanctions, procedure events
RecordStream(record_id)      lifecycle of records/notices/contracts
ItemStream(item_id)          ownership, possession, custody, movement, damage
RouteStream(route_id)        travel, patrols, risk, rumors, boundary events
RumorStream(topic_id)        propagation, mutation, contradiction, source loss
ControllerStream(id)         possession/debug binding metadata, not world truth
ProjectionStream(name)       rebuild/version/checkpoint metadata
```

Secondary streams must not contradict the world stream.

## Causal links

Causal links are typed edges, not prose decoration.

```yaml
CausalLink:
  from: evt_door_left_unlocked
  to: evt_item_removed_from_container
  relation: enabling_condition
  explanation: unlocked door allowed bedroom access without forcing entry
```

Allowed relation categories include:

- physical cause;
- enabling condition;
- actor intention;
- need pressure;
- value/trait/motive;
- belief or memory;
- observation;
- speech act;
- rumor;
- role obligation;
- norm implication;
- institutional procedure;
- relationship motive;
- resource constraint;
- scheduled continuation;
- interruption;
- random branch;
- environmental or regional process;
- LOD summary ancestry;
- debug/controller metadata outside world truth.

Debug tools must answer upstream and downstream causal questions, including why an action was possible, who observed it, which beliefs/records came from it, and where wrong suspicion originated.

## Projections

Projection families include:

- current component state;
- action-affordance indexes;
- scheduler state where needed for snapshots;
- actor belief stores;
- actor memory summaries;
- trace and observation indexes;
- institutional records;
- notice board state;
- ownership/custody indexes;
- route risk/public belief indexes;
- lead cards;
- embodied TUI view models;
- debug view models;
- causal graph indexes;
- story-sifter summaries;
- metrics/review reports;
- replay checksum and migration state.

Projection rebuild must be deterministic for a given event log, content version set, schema/upcaster registry, seed/random state, and projection version.

## Beliefs and records in the event log

Beliefs and records may have current projection forms, but important changes need event-level provenance when they can affect future action, testimony, institutional procedure, memory, or debug explanation.

```text
SpeechActCommitted
 -> ListenerObservedSpeech
 -> ListenerInterpretedClaim
 -> BeliefUpdatedFromSpeech
 -> RumorSeedCreated
```

Institutional records are world artifacts. Their lifecycle events must be committed.

## Save-package contract

A save package is the durable replay unit. It is not just current state.

```yaml
SavePackage:
  package_id: save_142_08_12_evening
  created_by_tool_version: tracewake-save-format@0.1
  event_log:
    segments:
      - world_000000_000999.events
      - world_001000_001999.events
    last_world_position: 18239
    event_log_checksum: sha256_or_equivalent
  content_manifest:
    ruleset: tracewake-core-rules@0.1
    domain_packs:
      - neutral_village@0.1
    fixture_or_seed: first_village_missing_property@0.1
    content_hashes: required
  schema_registry:
    event_schema_registry_version: 0.1
    supported_event_versions: declared
    upcaster_registry_hash: required
  snapshots:
    - id: snap_18000
      covers_world_position: 18000
      content_manifest_hash: required
      projection_versions: declared
      checksum: required
  projection_metadata:
    current_state: versioned_or_rebuild_required
    belief_projection: versioned_or_rebuild_required
    tui_view_models: rebuild_required
    debug_indexes: rebuild_required
  random_streams:
    seed_policy: declared
    streams:
      trace_generation: position_or_commitment
      perception_uncertainty: position_or_commitment
      agent_choice_noise: position_or_commitment
      institution_bias_noise: position_or_commitment
      regional_process: position_or_commitment
  replay_report:
    last_authoritative_replay: optional
    checksums: optional
    failures: none_or_report_refs
  diagnostics:
    validation_report_refs: []
    migration_report_refs: []
```

A save package must contain enough information to reject, migrate, or replay; not enough to guess.

## Snapshots

Snapshots improve performance. They do not replace causal ancestry.

Snapshots may include:

- current component state;
- scheduler queue;
- reservations;
- random stream positions;
- projection checkpoints;
- content/data manifest;
- LOD tier state;
- compaction references;
- checksum metadata.

Snapshot rules:

- declare exact event position covered;
- declare content/schema/projection versions;
- never erase ancestry needed by active traces, beliefs, records, investigations, wrong suspicions, leads, LOD promotions, or story sifting;
- be discardable and rebuildable.

## Event schema evolution and upcasters

Stored events are immutable.

Policy:

- every event has event type and schema version;
- non-breaking additions may use defaults;
- breaking changes require new event types, version-specific handlers, or upcasters;
- upcasters transform old representations at read/replay time without mutating stored events;
- upcaster chains are tested with historical fixtures;
- projections declare supported event versions;
- replay rejects unknown versions without a migration path.

```yaml
EventSchemaRegistryEntry:
  event_type: item_removed_from_container
  supported_versions: [1, 2]
  current_version: 2
  upcasters:
    - from: 1
      to: 2
      rule: add_reservation_context_from_defaulted_envelope
  replay_policy: reject_without_upcaster
```

## Replay modes and failure

### Authoritative replay

Used for saves, regression, and acceptance. Unknown event types, unsupported versions, missing content, checksum mismatch, random stream mismatch, invariant violations, projection drift, or missing causal references stop replay.

### Diagnostic replay

Used by debug tooling. It may continue after marked failures to inspect damage, but all derived state is non-authoritative.

### Projection-only rebuild

Rebuilds a projection from known-good events. Projection code changes are versioned and tested.

### Migration replay

Runs explicit event upcasters and/or content migration adapters and emits a migration report with before/after checksums.

Replay failure response:

```text
stop authoritative replay
record failure report
identify event position, projection, content/schema versions, expected/actual checksum
require migration, upcaster, or data fix before accepting save/log
```

Do not continue with best-effort guesses in authoritative mode.

## Random streams and meaningful draw audit

Tracewake uses scoped deterministic random streams. Avoid one global stream whose downstream results change when decorative draws are added.

Recommended streams:

```text
agent_choice_noise
perception_uncertainty
trace_generation
memory_decay
rumor_mutation
institution_bias_noise
regional_process
content_generation
```

A draw is meaningful if it can affect future world state, beliefs, traces, records, or institutional action.

```yaml
RandomDrawRecord:
  draw_id: rnd_000441
  stream: perception_uncertainty
  consumer: obs_elena_noise_resolution
  purpose: heard_noise_confidence
  event_context: evt_container_open_attempt
  sequence: 91
  result_category: low_confidence_noise
  value_policy: raw_in_debug_hash_or_commitment_elsewhere
```

## Compaction and summary ancestry

Long simulation permits compaction through summary events, not hidden edits.

Preserve detailed ancestry for:

- ownership/possession/custody changes;
- active traces;
- belief-changing events;
- contradiction events;
- records, reports, testimony, notices, contracts, sanctions;
- crimes and suspected crimes;
- injuries/deaths;
- active leads;
- events referenced by records, speech, memory, or institutions;
- LOD promotion/demotion causes;
- random branches that materially changed outcomes.

Summarize low-salience routine spans only when active causal ancestry is preserved or safely summarized.

## Storage-engine stance

The architecture requires append-only event semantics, replay, snapshots, projection rebuild, schema evolution, and save-package manifests. It does not require a final storage engine now.

Acceptable early storage:

```text
append-only event segments
+ content manifest
+ schema/upcaster registry manifest
+ snapshot files
+ projection cache files
+ replay checksum reports
```

Possible later storage:

```text
embedded database or event-store adapter
+ indexed streams
+ snapshot store
+ projection store
+ migration/upcaster registry
```

The storage adapter must not become the domain model.

## Acceptance implications

Event/replay/save features must test:

- append-only event commit;
- event envelope versioning;
- causal link queries;
- projection rebuild determinism;
- snapshot equivalence to replay from genesis for covered state;
- save-package content/schema mismatch failure;
- event upcaster fixtures;
- unknown event version loud failure;
- random stream audit;
- replay of no-human run;
- active ancestry preservation after compaction;
- debug explanation from event/causal graph.

## Anti-patterns

- Saving only current mutable state.
- Editing old events to repair history.
- Quest completion flags as truth.
- Projection cache treated as authority.
- Event without causes for meaningful mutation.
- Belief without source.
- Notice without author, issuer, claims, and posting event.
- Random outcome without seed discipline.
- Snapshot erases active causal ancestry.
- Replay silently skips unknown events.
- Storage schema dictates simulation semantics.
