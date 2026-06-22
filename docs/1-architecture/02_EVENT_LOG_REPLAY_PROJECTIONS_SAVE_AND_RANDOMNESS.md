# Event Log, Replay, Projections, Save, and Randomness

## Status

Authoritative architecture contract.

## Purpose / core rule

Event sourcing is Tracewake's forensic backbone. Authoritative current state is a projection of committed events plus validated initial seeds. Replay must be able to rebuild meaningful state, diagnostics, and projections, or fail loudly with a typed reason.

Current state answers “what is true now.” The event log answers “how did this become true, who could know it, and what evidence proves it.”

## Authority owned

The event/replay architecture owns:

- event envelope requirements;
- event stream boundaries;
- projection rebuild rules;
- save manifests;
- event schema versioning and upcasting policy;
- random stream discipline;
- deterministic replay gates;
- decision/stuck/provenance artifacts as replay material.

## Authority denied

Event sourcing may not be reduced to a debug log. Projections may not become truth writers. Snapshots may not erase forensic ancestry. Migration may not silently discard old events. Randomness may not be hidden behind runtime libraries.

## Contract

### Event envelope minimum

Every committed event must carry enough information to support replay and audit:

```text
event_envelope:
  event_id
  event_type
  event_schema_version
  stream
  global_order
  stream_position
  sim_tick_or_tick_window
  ordering_key
  actor_id_or_process_id
  participants
  causes
  content_manifest_id
  random_stream_ref_or_none
  payload
  checksum_before_or_after_when_applicable
```

Events are immutable after commit. If a historical event was wrong, correction is another event, not mutation of history.

### Authoritative event/replay time

Event/replay time is the ordered substrate for validation, scheduling due effects, duration accounting, replay, projection rebuild, and causal explanation. It may prove ordering and consequence legality; it is not itself evidence that an actor, household, institution, group, or region knew the temporal fact.

Temporal facts leave this substrate for holder-facing cognition, procedure, affordance selection, speech interpretation, lead interpretation, view models, or LOD promotion only through events, records, procedures, or projections that preserve modeled acquisition ancestry. A projection may summarize temporal facts, but the summary must remain traceable to the event, record, procedure, or observation channel by which the holder could know it.

Projection rebuilds, snapshots, and compaction certificates must preserve temporal ancestry. They may not replace event time, duration boundaries, record/procedure timing, or holder acquisition timing with a raw current-time label that erases why the temporal state exists or who could know it.

Replay diagnostics must distinguish temporal divergence classes, including wrong event ordering, missing duration terminals, due-effect drift, unrecorded wall-clock input, and unsupported temporal migration. Those diagnostics are replay/debug evidence unless a modeled channel separately makes a temporal fact holder-known.

Every accepted world step, including an otherwise-empty step, needs
replay-visible tick-boundary ancestry sufficient to rebuild the temporal
frontier and explain why the frontier advanced. Acceleration is represented as
repeated one-tick ancestry, never as a silent multi-tick skip. The boundary
evidence is world/replay material; it does not become holder-known temporal
knowledge without a modeled acquisition channel.

### Event streams

Tracewake may have distinct streams such as world, agent, epistemic, institution, controller, diagnostic, debug, content-load, LOD, and save-manifest. Stream separation is for authority and replay discipline, not for hiding causality. Cross-stream causes must be explicit.

Controller binding is not world state. Debug inspection is not world state. Actor beliefs are not physical truth. Institutional records are not truth. Every stream must preserve these distinctions.

### Current state is projection

Authoritative state is rebuilt from:

1. validated content/fixture seeds;
2. committed events in deterministic order;
3. replayable migrations/upcasters for old event shapes;
4. deterministic boundary inputs recorded in manifests.

No projection may create a fact that was not implied by validated seeds plus events. Read models, view models, metrics, notebooks, maps, story summaries, and debug views are projections.

### Save package contract

A save package must include:

- content manifest ID and version;
- event log or event segments;
- event schema registry versions;
- projection schema versions;
- random stream registry and consumed draw ranges;
- initial seed identity and checksum;
- replay checkpoint/snapshot metadata, if any;
- upcaster/migration manifest;
- diagnostic artifact manifest;
- client/transcript artifacts needed for acceptance, when relevant.

Snapshots are acceleration artifacts. They are invalid without the event ancestry or a verified compaction certificate that preserves all required forensic claims.

### Event schema evolution

Event envelopes carry event type and schema version. Old events remain readable through version-specific handlers or explicit upcasters. Replay must fail loudly when:

- an event type is unknown;
- a schema version lacks a reader/upcaster;
- a payload cannot be validated;
- a migration loses a required field;
- a migration cannot preserve provenance, random draw, or causality.

Silent best-effort migration is forbidden. Long-lived saves are a product requirement, not polish.

### Random stream discipline

Randomness is allowed only when scoped, deterministic, and auditable.

A random draw must identify:

```text
random_draw:
  stream_id
  purpose
  seed_or_parent_stream
  draw_index
  algorithm_version
  consumer_context
  committed_result_or_replayable_derivation
```

Separate random streams are required for independent concerns: content seeding, social noise, observation uncertainty, regional summary variation, and language-surface variation. A changed display string must not shift world simulation randomness.

Where exact determinism matters, commit the draw result. Where replayable derivation is enough, the stream and draw index must be stable.

### Decision and diagnostic replay

The event log must preserve or reconstruct:

- holder-known context packet identity and provenance summary;
- candidate generation inputs and rejected candidates;
- intention lifecycle changes;
- HTN/routine method selection and rejected methods;
- local plan attempts, chosen proposals, failed proposals, and fallback decisions;
- validation reports with actor-visible and debug-only fields separated;
- stuck diagnostics with typed blocker category and actor-known explanation;
- hidden-truth comparison markers that remain debug-only.

The goal is not merely “the same final state.” The replay must be able to prove that the same kind of decision was made from the same authorized knowledge boundary.

### Projection rebuild

Projection rebuilds must be deterministic and separately testable. Important projections include:

- physical state;
- agent state: needs, intentions, routines, decision traces, stuck diagnostics;
- epistemic state: observations, beliefs, contradictions, notebooks;
- institutional state: records, procedure status, roles, notices, sanctions;
- view models;
- metrics;
- story-sifting summaries;
- debug comparison views.

Interval projections produced after world-step controls rebuild
deterministically from source-bearing frontier deltas, not from client-local
state or raw hidden-world diffs. Stop decisions for continuation and
advance-until controls must replay from typed step evidence and holder-known
salience inputs, or fail loudly as projection/replay divergence.

Projection errors are reportable artifacts, not console messages.

## Acceptance implications

Acceptance requires at least:

- event append order is deterministic;
- live state and replay-rebuilt state match for relevant projections;
- missing/reordered/unsupported events are detected;
- event schema migration failures are loud;
- random streams do not drift because of view/client/debug choices;
- actor/institution decisions are replayable as decisions, not just as resulting actions;
- no-human metrics are byte-identical after log serialization/replay;
- debug artifacts can compare truth and belief without contaminating embodied surfaces.

## Anti-patterns

- “We can snapshot and drop old events because the current state is correct.”
- “The event log is only for debugging.”
- “The replay matched position, so cognition was valid.”
- “Randomness is fine because the seed is global.”
- “The migration can fill missing provenance with unknown-but-accepted.”
- “The story summary can cache a culprit field because it is just a projection.”

## Cross-document obligations

- Document 03 requires context packets and provenance to be replay-visible.
- Document 04 requires every proposal outcome to emit or reference validation evidence.
- Document 05 requires decision traces and stuck diagnostics to be typed and replayable.
- Document 10 requires TUI transcripts and debug views to remain derived projections.
- Document 13 defines the acceptance artifacts that prove replay integrity.
