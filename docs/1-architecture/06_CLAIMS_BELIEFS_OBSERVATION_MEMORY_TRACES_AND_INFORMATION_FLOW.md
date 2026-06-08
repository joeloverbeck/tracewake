# Claims, Beliefs, Observation, Memory, Traces, and Information Flow

## Status

Authoritative architecture contract.

## Purpose / core rule

Typed propositions are Tracewake's epistemic currency. Observations, beliefs, memories, records, rumors, leads, notices, reports, and diagnostics are source-bound carriers of typed claims. They may be wrong, stale, private, biased, contradicted, or uncertain.

Observation is not interpretation. Evidence is not certainty. Record is not truth. Report is not proof. Violation is not detection.

## Authority owned

This document owns epistemic data flow, proposition shape, observation/belief distinctions, source/provenance rules, privacy/holder filtering, memory, rumors, records as claim carriers, absence evidence, traces, and display-string limitations.

## Authority denied

The epistemic layer may not rewrite physical truth. It may not make a holder know a claim without a modeled source. It may not promote debug truth into belief. It may not treat a rendered sentence as authoritative proof.

## Contract

### Typed proposition minimum

A proposition must be structured enough for validation, filtering, replay, and contradiction detection.

Examples:

```text
item_located_in_container(item_id, container_id)
item_missing_from_expected_location(item_id, expected_location)
actor_at_place(actor_id, place_id)
food_supply_accessible(food_supply_id)
workplace_assignment(actor_id, workplace_id)
record_posted(record_id, board_id)
report_received(institution_id, report_id)
obligation_due(holder_id, obligation_id)
sound_heard_near_place(place_id)
possible_movement_near_place(place_id)
```

A display sentence such as “Mara probably stole the coins” is not a proposition unless represented as a typed claim with source, stance, holder, and uncertainty.

### Observation

An observation records that a holder had a sensory/contact/channel event. It includes observer, channel, time/window, place, target, source event, confidence, alternatives, and privacy scope.

Observation does not automatically create guilt, certainty, or global knowledge. A simple sound can support “sound heard near room,” not “Mara stole item,” unless further modeled evidence supports that claim.

### Belief

A belief is a holder's stance toward a proposition. It includes holder, proposition, stance, confidence, source, channel, acquired tick, last verified tick, stale-after tick, observation links, contradiction links, and privacy scope.

Belief stances include believes true, believes false, expects true, plausible, doubts, and unknown/unresolved. Use belief, not knowledge, when correctness is not guaranteed.

### Memory

Memory is not a perfect database. It is a modeled source of claims. Memories can decay, become stale, conflict, be misremembered, or be shaped by role/relationship. Memory still requires provenance: original observation, testimony, record, institution procedure, or prehistory seed.

### Records and artifacts

Records, notices, ledgers, letters, marks, contracts, role rosters, posted warnings, and institutional forms are artifacts carrying typed claims. A record can be forged, stale, incomplete, lost, hidden, private, public, or disputed.

A record creates a claim source, not truth. Institutions may act on records because institutions are record-using machines, but records remain fallible.

### Rumors and testimony

Rumors and testimony are speech-derived claims. They must preserve speaker, listener, channel, time, proposition, confidence/trust stance, and possible distortion. Hearsay is allowed; omniscient fact transfer is not.

### Absence as evidence

Absence can become evidence only after a modeled search/observation event over an expected scope. “The strongbox was checked and the expected coins were absent” can support a missing-property belief. It cannot prove who moved them.

### Leads

A lead is a holder-known or projection-known direction for attention, not a quest step and not truth. Leads carry source, proposition, confidence, staleness, eligible holders, and allowed next ordinary actions.

### Trace and erasure

Physical and social actions can create traces: sound, footprints, opened doors, missing items, moved objects, written notes, smell, witness memory, institutional logs. Traces can decay or be erased through events. Erasure is not retroactive deletion of event history; it changes future observability.

### Privacy and filtering

Every observation, belief, memory, record, and notebook entry has a holder/visibility scope. Embodied views filter by holder-known context. Debug views may inspect broader truth only with non-diegetic markers and structural separation.

## Data examples

Missing-property chain:

```text
record: actor_tomas expects coin_stack_01 in strongbox_tomas
observation: actor_tomas checks strongbox_tomas at tick 40
observation_result: coin_stack_01 absent from checked container
belief: actor_tomas believes coin_stack_01 missing from expected location
lead: source-bound possible lead: ask household / inspect room / report missing property
```

Forbidden shortcut:

```text
belief: actor_tomas believes actor_mara stole coin_stack_01
source: physical truth says item is at actor_mara
```

## Required diagnostics / replay / TUI hooks

- Epistemic projection rebuild must be deterministic and checksummed.
- Notebook views must be actor-scoped and source-bound.
- Contradictions must link prior expectation belief and contradicting observation.
- Debug truth/belief mismatch views must be debug-only.
- Tests must assert typed propositions and source links, not only rendered text.
- Decision traces must list the belief/observation/known-place inputs used.

## Acceptance implications

The first missing-property proof requires a holder-known expectation, a modeled absence observation, a belief/contradiction update, actor-visible notebook/lead output, and debug comparison showing truth without leaking it. A test that jumps from missing item to culprit fails.

## Anti-patterns

- “Observation of absence proves theft.”
- “A record says it, so it is true.”
- “The actor has no belief entry, so the actor knows nothing and the planner is safe.”
- “Debug mismatch can be rendered in embodied notebook because testers need it.”
- “A rumor is just a string.”
- “The lead points to the true next step by design.”

## Cross-document obligations

This document feeds holder-known contexts in document 03, actor decisions in document 05, speech acts in document 07, institutions in document 08, incidents/leads in document 11, LOD promotion in document 12, and acceptance artifacts in document 13.
