# Validation, Testing, and Acceptance Architecture

## Status

This document defines validation and testing gates for every runnable Tracewake phase.

Validation is not a late QA activity. It is architecture. Tracewake's premise fails without deterministic replay, actor-knowledge filtering, no-player parity, event causality, and TUI playability.

## Core rule

Every feature must be testable in no-human simulation, embodied TUI play, replay, and debug inspection unless explicitly marked as debug-only or future-only.

## Required gates for every runnable phase

Each runnable phase must include gates for:

- no-human simulation;
- TUI playability;
- actor-knowledge filtering;
- possession parity;
- no player privilege;
- no hidden ground-truth leakage;
- deterministic replay;
- event/projection rebuild;
- causal ancestry inspection;
- event/trace creation;
- belief provenance;
- institutional fallibility where relevant;
- no-script compliance;
- LLM-disabled operation;
- action registry parity;
- data/schema validation;
- event schema migration/upcaster tests;
- automated regression tests;
- TUI/view-model tests.

## Universal feature questions

Before acceptance, every feature must answer:

- What caused it?
- Who knows it?
- How can they be wrong?
- What traces exist?
- What institution, norm, household, role, or record cares?
- Can an NPC do the same kind of thing?
- Can it be played and inspected through the TUI?
- Can it run in a no-human simulation?
- Can debug mode explain it?
- Does it leak player privilege, ground truth, genre assumptions, scripting, or LLM authority?
- Can it replay and rebuild projections deterministically?

## Test layers

### Content validation tests

Validate:

- syntax and schema;
- stable IDs;
- references;
- action definitions;
- affordance exposure;
- norms/procedures;
- HTN methods;
- speech templates;
- record schemas;
- scenario seeds;
- no outcome chains;
- no player-only verbs;
- domain pack boundary compliance;
- content version manifest.

### Kernel/action tests

Validate:

- command/proposal intake;
- actor authority;
- physical preconditions;
- knowledge preconditions;
- social/normative preconditions;
- reservation conflicts;
- duration scheduling;
- interruption;
- outcome resolution;
- failure events;
- event commit;
- trace and observation hooks;
- projection update.

### Epistemic tests

Validate:

- observation vs interpretation;
- belief source/confidence;
- expectation contradiction;
- absence-as-evidence;
- rumor mutation;
- stale information;
- record-derived beliefs;
- lies vs hallucinations;
- actor notebook filtering;
- debug truth/belief mismatch.

### Agent tests

Validate:

- need pressures;
- routine as defeasible intention;
- project persistence;
- HTN method selection;
- bounded local planner budget;
- action proposal generation;
- replanning triggers;
- failure handling;
- no ground-truth planning;
- planner trace output.

### Institution tests

Validate:

- roles/permissions/obligations;
- household access/privacy;
- norm classification;
- detection vs violation;
- report lifecycle;
- ledger/record artifact;
- evidence thresholds;
- procedure failure;
- stale records/notices;
- bias/corruption as causal events if modeled;
- institutional knowledge filtering.

### Scheduler/replay tests

Validate:

- discrete time;
- stable ordering;
- no wall-clock decisions;
- scheduled action completion;
- interruption windows;
- reservation release;
- seeded random streams;
- meaningful random draw records;
- replay checksum;
- projection rebuild;
- snapshot load;
- event schema upcasting;
- migration failure handling.

### TUI/view-model tests

Validate:

- embodied view hides truth;
- debug view shows truth;
- action menus derive from affordances;
- why-not explanations derive from validation;
- actor notebook source filtering;
- possession switch does not transfer knowledge;
- no player-only mutation;
- TUI input transcript replay;
- no graphical dependency.

### LLM/language tests

For v1 deterministic templates:

- structured speech acts commit;
- templates preserve uncertainty;
- unsupported claims absent;
- LLM-disabled mode is normal.

For future LLM surfaces:

- prompt packets are actor-filtered;
- output validates against structured contract;
- invalid output is rejected/repaired;
- deterministic mocks cover tests;
- live output never decides authoritative state.

## Canonical acceptance scenario: missing-property miracle

The first major scenario test should run both no-human and embodied variants.

Expected causal chain:

```text
Tomas expects coins in strongbox
 -> Mara has debt pressure and believes coins may be there
 -> Mara gets opportunity or chooses search/theft attempt through action pipeline
 -> action produces events, traces, and possible observations
 -> Tomas later checks strongbox
 -> expectation contradiction occurs
 -> Tomas forms candidate goals
 -> Tomas may search, ask, report, accuse, or conceal
 -> clerk may receive report and create ledger entry
 -> wrong suspicion may arise from partial sources
 -> debug explains truth/belief/record mismatch
```

The test passes only if:

- no player-only actions are used;
- no actor receives hidden truth;
- theft/movement uses typed object affordances;
- action duration/reservation/interruption are modeled where relevant;
- traces are created or explicitly not created for causal reasons;
- witnesses have partial observations;
- belief provenance exists;
- institution records partial claims, not truth;
- replay rebuilds projections;
- TUI can inspect actor-known state;
- debug can inspect full causal graph.

## No-human simulation gate

A no-human gate should run for at least several simulated days once ordinary routines exist.

It must produce meaningful ordinary-life events:

- meals;
- sleep;
- work;
- household movement;
- conversations;
- storage use;
- expectation checks;
- reports/notices if warranted;
- wrong beliefs/stale information where circumstances create them;
- no protagonist gravity.

Metrics to capture:

- significant events;
- routine events;
- action rejections/failures;
- belief updates;
- contradictions;
- reports/records/notices;
- trace creation/discovery;
- planner failures;
- stuck actors;
- replay failures;
- projection rebuild time;
- player-conditioned event rate in embodied runs.

## No-player privilege gate

For every world-affecting action exposed to the TUI:

- an AI actor under equivalent conditions can propose it;
- the same action definition is used;
- the same preconditions are checked;
- the same events are committed;
- the same traces/observations/norm hooks apply;
- rejection/failure is equivalent.

If a player-only action is necessary for debugging, it must live in debug tooling and be impossible in embodied play.

## Actor-knowledge gate

Run the same scenario from multiple possessions.

Example:

- possess Tomas before discovering missing coins;
- possess Mara after hiding coins;
- possess Elena after hearing noise;
- possess clerk after receiving report;
- inspect actor notebook for each;
- verify no notebook contains another actor's private knowledge;
- verify debug view can show mismatch.

## Deterministic replay gate

Required checks:

- same seed/event log produces same projections;
- replay from snapshot equals replay from genesis for covered state;
- projection rebuild is deterministic;
- random stream positions match;
- event schema version fixtures replay;
- upcaster tests cover old event examples;
- unknown event version fails loudly;
- content version mismatch fails or migrates explicitly.

## No-script compliance gate

Scan content and runtime behavior for hidden outcome chains.

Forbidden patterns:

- forced sequence of world outcomes;
- quest completion flags;
- NPC waits for player acceptance;
- objective truth exposed by lead;
- reward spawn from completion;
- domain pack direct event write;
- drama manager changes probability;
- LLM-created fact accepted.

Allowed:

- scenario pressures;
- fixture setup for tests;
- authored action machinery;
- authored norms/procedures;
- authored records and starting beliefs;
- summary events with causal ancestry.

## Regression artifacts

Automated runs should produce artifacts useful for review:

- event log segment;
- replay checksum report;
- projection rebuild report;
- causal graph excerpt;
- actor belief diffs;
- TUI transcript;
- why-not log;
- content validation report;
- scheduler queue snapshots;
- planner trace samples;
- LLM validation report if applicable.

These artifacts should be inspectable without a graphical client.

## Acceptance levels

### Architecture accepted

The feature has a boundary contract and does not violate doctrine.

### Prototype accepted

The feature runs in deterministic fixture tests and exposes debug inspection.

### First-slice accepted

The feature runs in no-human simulation, TUI embodied play, replay, projection rebuild, and actor-knowledge tests.

### Research-grade accepted

The feature supports metrics, reproducible runs, stress tests, LOD/replay where relevant, and falsifiable validation reports.

## Anti-patterns

- Manual demo accepted without no-human run.
- TUI works by reading truth.
- Tests depend on live LLM output.
- Save/load test ignores event replay.
- Projection drift ignored.
- Fixture bypasses action pipeline for normal behavior.
- Actor planner has no trace output.
- Institution tests assert truth instead of institutional belief.
- Content validation stops at syntax.
- Debug tools cannot explain why an event happened.
