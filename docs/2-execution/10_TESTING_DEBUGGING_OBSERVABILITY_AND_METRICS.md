# Testing, Debugging, Observability, and Metrics

## Core claim

Emergent simulation without forensic testing becomes superstition.

Every feature must answer:

```text
What caused it?
Who knows it?
How can they be wrong?
What traces exist?
What institution or norm cares?
Can an NPC do the same kind of thing?
Can it be played and inspected through the TUI?
Does it leak player privilege, ground truth, genre assumptions, scripting, or LLM authority?
Can it run in no-human simulation?
Can debug explain it?
Can replay rebuild it?
```

If a feature cannot answer these questions, it is not accepted.

## Canonical testing matrix

| Area | Minimum proof |
|---|---|
| Action pipeline | proposal, validation, rejection/failure, event commit, trace hooks |
| Event sourcing | append-only event, stable order, replay, projection rebuild |
| TUI/view model | actor-filtered embodied view, debug truth view, semantic action selection |
| Epistemics | observation, belief source, confidence, contradiction, no hidden truth |
| Possession parity | controller binding only, no knowledge transfer, no player-only verbs |
| Needs/routines | hunger/fatigue/safety affect choices, routines fail/interruption visible |
| Institutions | reports and records from claims/procedure, not ground truth |
| Data validation | stable IDs, references, no scripts, no player schema, source-backed beliefs |
| No-human simulation | ordinary life runs without controller, no protagonist gravity |
| Debug | causal graph, planner trace, belief mismatch, record provenance, why-not |
| LLM boundary | deterministic mocks/templates; no live output in authoritative tests |

## Per-phase minimums

### Phase 0

Required tests/reviews:

- vocabulary review;
- manual causal trace review;
- no-scripting review;
- view-model contract review;
- golden scenario paper review;
- data fixture review;
- first-proof/second-proof boundary review.

No executable tests are required, but every future testable claim must be mapped.

### Phase 1

Required automated tests:

- unit tests for action validation and event application;
- property tests for item possession and no player-only action registry;
- integration tests for door/container/item movement;
- replay tests for item location;
- TUI/view-model acceptance for local action menu;
- no-human scheduler advance;
- debug event log and projection rebuild.

### Phase 2

Required automated tests:

- observation vs interpretation;
- belief holder/source/confidence;
- expectation contradiction;
- actor-known filtering;
- embodied/debug view split;
- possession switch without knowledge transfer;
- why-not knowledge blocker;
- replay belief projection;
- epistemic leakage regression.

### Phase 3

Required automated tests:

- needs thresholds and effects;
- sleep/eat/work scheduling;
- routine applicability/failure/interruption;
- intention persistence;
- no-human day;
- planner trace output;
- TUI wait/sleep/work/continue controls;
- replay routine projection;
- stuck actor diagnostics.

### Phase 4

Required automated tests:

- report speech-act validation;
- office availability;
- record creation/provenance;
- institution non-omniscience;
- household access/privacy;
- norm classification vs detection/proof;
- wrong suspicion;
- record actor-known view;
- institution debug inspector;
- first proof full-chain acceptance.

### Second proof

Required before start:

- all first-proof tests passing;
- regression tests preventing notice/travel systems from bypassing first-proof rules;
- first-proof golden fixtures preserved.

## Unit tests

Unit tests cover local rules.

Minimum categories:

- action preconditions;
- event envelope creation;
- event application;
- component mutation through events;
- stable ordering;
- ownership/possession/custody invariants;
- locks/doors/containers;
- traces;
- observation creation;
- belief update;
- expectation contradiction;
- confidence/source handling;
- norm classification;
- speech-act validation;
- report record creation;
- view-model filtering;
- why-not reason generation;
- routine applicability;
- scheduler wakeup;
- replay failure modes.

## Property tests

Required properties:

- no item has two possessors;
- every possession/custody change has an event;
- no important belief lacks holder/source;
- no important record lacks author/provenance/artifact;
- no institutional case lacks report/observation/inference/procedure source;
- no guard knows unseen crime unless informed, observing, inferring, or reading;
- no player-only world action exists in the core registry;
- no embodied view model contains debug-only truth;
- no speech act commits unsupported facts unless classified as lie/speculation with actor motive/basis;
- no action succeeds after precondition rejection;
- replay of same seed/event log produces same projection;
- no-human events contain no player identity;
- every phase mechanic has TUI/view-model reachability;
- LLM-disabled operation remains normal.

Useful candidate tooling may include proptest or equivalent. The property matters more than the crate.

## Golden scenario tests

Golden scenarios use deterministic fixtures and assert occurrence and non-occurrence.

Required golden scenarios:

### strongbox_001

Assert:

- Mara may remove/move coins only through action pipeline;
- item movement event exists;
- Tomas does not know until modeled channel;
- Elena's observation, if any, is uncertain and source-backed;
- debug causal graph explains chain;
- embodied view hides hidden truth.

Also assert non-occurrence:

- no case record exists before report;
- no guard knows culprit from true event;
- no quest state appears.

### expectation_contradiction_001

Assert:

- expectation source exists;
- check/search creates observation;
- absent expected item creates contradiction;
- no expectation means no contradiction;
- candidate goals are generated from belief/motive, not script.

### possession_parity_001

Assert:

- switching body changes input binding only;
- new actor notebook lacks previous actor knowledge;
- previous actor continues ordinary state;
- player-only action registry is empty;
- debug possession history remains non-world metadata.

### report_record_001

Assert:

- report is structured speech act;
- receiver availability matters;
- record created from report claims;
- institution does not learn ground truth;
- record has physical/provenance artifact;
- report can be refused/delayed.

### wrong_suspicion_001

Assert:

- wrong suspicion has visible source chain;
- suspicion threshold is actor/institution-specific;
- correct culprit not required;
- suspicion can be contradicted later;
- debug explains why suspicion is wrong.

### no_human_day_001

Assert:

- village runs without controller;
- agents sleep/eat/work/rest/speak minimally;
- no player identity appears;
- routines can fail;
- debug attach afterward works;
- replay rebuilds run.

### view_filtering_001

Assert:

- embodied view hides ground truth;
- debug view reveals truth;
- different actors get different actor-known views;
- hidden truth never appears through actor notebook.

### replay_rebuild_001

Assert:

- event log rebuilds state and projections;
- content version mismatch fails or migrates explicitly;
- unknown event version fails loudly;
- causal ancestry remains inspectable.

## Fuzz and randomized seed tests

Run small villages across seeds after Phase 3.

Check:

- planner loops;
- reservation deadlocks;
- impossible paths;
- action rejection storms;
- stuck actors without cause;
- runaway rumor/belief growth;
- belief-store explosion;
- stale info correction bugs;
- actor starvation/sleeplessness from routine bugs;
- no-human protagonist gravity;
- replay nondeterminism;
- player-conditioned event anomalies;
- LOD ancestry problems when introduced.

Random tests do not replace golden tests. They find cracks after contracts exist.

## TUI/view-model tests

Every runnable phase needs TUI-level or view-model-level tests.

Required harness abilities:

- load deterministic fixture;
- bind controller to actor;
- inspect embodied view model;
- select action by stable semantic ID;
- advance scheduler;
- assert committed events;
- assert actor notebook changes;
- assert hidden truth absent;
- switch to debug view;
- assert truth/causal graph available;
- replay input transcript.

Tests should not depend on terminal coordinates or visual styling.

## Epistemic leakage tests

Scenarios:

- player knows truth, actor does not;
- actor attempts truthful accusation without actor-known support;
- actor attempts reckless accusation if modeled;
- map/lead marker shows believed location, not true location;
- dialogue cannot reveal hidden truth;
- institution does not read player/debug notebook;
- possession history does not alter beliefs;
- debug view state never appears in embodied view;
- LLM prompt packet excludes hidden truth if future LLM surface exists.

## Institution tests

Assert:

- report receiver exists;
- office access/availability matters;
- record has author and artifact;
- ledger entry cites speech/report source;
- institution belief derives from record/speech/observation/inference;
- violation/detection/suspicion/proof/sanction are separate;
- wrong suspicion can form;
- case may remain open/stale/wrong;
- institutional procedure can fail.

Forbidden pass:

```text
case exists because theft happened
guard suspects culprit because culprit truly did it
```

Required pass:

```text
case exists because Anna received a report and wrote a ledger entry
guard suspects actor because institutional claims and threshold support suspicion
```

## Replay/projection rebuild tests

Replay reconstructs:

- state at tick;
- event sequence;
- random draw audit records;
- action proposals;
- precondition reports;
- belief updates;
- planner decisions;
- institution decisions;
- speech-act commits;
- record projections;
- TUI view model snapshots;
- debug causal graph.

Replay must fail loudly on:

- unknown event type;
- unsupported event schema version;
- missing content version;
- projection checksum mismatch;
- nondeterministic ordering;
- random stream mismatch;
- missing causal reference;
- invariant violation.

No silent repair in authoritative replay.

## LLM-disabled and LLM-boundary tests

For first proof:

- deterministic templates work;
- no live LLM call is used;
- tests do not depend on generated wording;
- output text is surface only.

For future LLM surfaces:

- prompt packets are actor-filtered;
- unsupported claims are rejected or downgraded;
- uncertainty is preserved;
- structured speech acts are the commit authority;
- live output cannot mutate state;
- deterministic mocks cover all acceptance tests.

## Debug inspector requirements

Debug mode must answer:

- why did this action become available?
- why was this action rejected?
- why did this event happen?
- what caused this belief?
- why does this actor not know the truth?
- what traces exist and what do they support?
- why did this routine fail?
- why did this report create a record?
- why does the institution suspect the wrong actor?
- what did replay rebuild?
- where did projection drift occur?

Debug output may expose truth only through non-diegetic view models.

## Metrics

Track from Phase 1 onward where relevant:

- significant events/day;
- routine events/day;
- causal coverage;
- events missing cause;
- action proposals/rejections/failures;
- belief updates;
- contradictions;
- stale beliefs;
- trace creation/discovery/decay;
- reports attempted/received/refused/delayed;
- records opened/amended/contradicted;
- wrong suspicions;
- planner time;
- planner failures;
- stuck actors;
- no-human run length;
- replay failures;
- projection rebuild time;
- TUI action coverage;
- player-conditioned event rate;
- LLM validation failures if enabled.

Metrics are not vanity. They are alarms for broken emergence.

## Failure as deliverable

Every phase must deliberately test failure.

Examples:

- locked container;
- actor cannot perceive item;
- actor lacks knowledge to accuse;
- actor lacks permission;
- actor unwilling to violate norm;
- actor too tired/hungry;
- office closed;
- clerk absent;
- report refused/delayed;
- guard lacks evidence threshold;
- suspicion threshold not met;
- suspicion threshold met for wrong reason;
- container search finds nothing;
- trace ambiguous;
- event rejected by preconditions;
- action interrupted;
- no route/path;
- replay mismatch;
- view model attempts debug truth leak.

Failure cases prove causal machinery. A system that only succeeds is probably scripted.

## Anti-patterns

Reject:

- passing headless tests while TUI cannot play feature;
- testing only success paths;
- belief tests checking truth rather than holder/source;
- live LLM calls in golden tests;
- debug assertions relying on player identity;
- institution tests reading ground truth;
- replay that cannot explain why;
- metrics ignored until scale breaks;
- fuzzing before deterministic golden fixtures exist;
- UI tests selecting by screen coordinates;
- content validation stopping at syntax;
- debug tools with no causal explanation.
