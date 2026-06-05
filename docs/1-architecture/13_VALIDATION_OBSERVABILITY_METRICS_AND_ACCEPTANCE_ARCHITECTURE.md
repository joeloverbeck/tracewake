# Validation, Observability, Metrics, and Acceptance Architecture

## Status

This document defines validation gates, observability contracts, metrics, review artifacts, and acceptance levels for runnable Tracewake phases.

Validation is architecture, not late QA. Tracewake fails without deterministic replay, actor-knowledge filtering, no-player parity, event causality, no-human simulation, and TUI playability.

## Core rule

Every feature must be testable in no-human simulation, embodied TUI play, deterministic replay, projection rebuild, and debug inspection unless explicitly marked as debug-only or future-only.

A manual demo is not acceptance.

## Authority

This subsystem owns:

- required validation gates;
- test layer categories;
- no-human run requirements;
- deterministic replay checks;
- actor-knowledge gates;
- no-player privilege gates;
- content validation gates;
- observability artifact contracts;
- metrics and review reports;
- acceptance level definitions.

It is denied:

- accepting systems with no replay;
- accepting TUI that reads truth;
- accepting live-LLM-dependent behavior;
- accepting content validated only by syntax;
- accepting phase claims without review artifacts;
- turning execution tickets into architecture.

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
- event schema migration/upcaster tests where events persist;
- automated regression tests;
- TUI/view-model tests.

## Universal feature questions

Before acceptance, every feature must answer:

- What caused it?
- Who knows it?
- How can they be wrong?
- What traces exist?
- What institution, norm, household, role, record, public artifact, or relationship cares?
- Can an NPC do the same kind of thing?
- Can it run in no-human simulation?
- Can it be played and inspected through the TUI?
- Can debug mode explain it?
- Can replay and projection rebuild reconstruct it deterministically?
- Does it avoid player privilege, ground truth leakage, genre leakage, scripting, hidden drama direction, quest ontology, and LLM authority?

## Test layers

### Content validation tests

Validate:

- syntax and schema;
- stable IDs;
- references;
- deterministic loading order;
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
- content version manifest;
- fixture coverage.

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

### Scheduler/replay/save-package tests

Validate:

- discrete time;
- stable ordering;
- no wall-clock decisions;
- scheduled action completion;
- interruption windows;
- reservation release;
- scoped seeded random streams;
- meaningful random draw records;
- save-package manifests;
- content/schema mismatch failure;
- replay checksum;
- projection rebuild;
- snapshot load;
- event schema upcasting;
- migration failure handling.

### Epistemic tests

Validate:

- observation versus interpretation;
- typed belief source/confidence;
- expectation contradiction;
- absence-as-evidence;
- rumor mutation;
- stale information;
- record-derived beliefs;
- lies versus hallucinations;
- actor notebook filtering;
- debug truth/belief/record mismatch.

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
- planner trace output;
- agent LOD promotion honesty.

### Institution tests

Validate:

- roles/permissions/obligations;
- household access/privacy;
- norm classification;
- detection versus violation;
- report lifecycle;
- ledger/record artifact;
- evidence thresholds;
- procedure failure;
- stale records/notices;
- bias/corruption as causal events if modeled;
- institutional knowledge filtering.

### TUI/view-model tests

Validate:

- embodied view hides truth;
- debug view shows truth visibly;
- action menus derive from affordances;
- why-not explanations derive from validation;
- actor notebook source filtering;
- possession switch does not transfer knowledge;
- no player-only mutation;
- TUI input transcript replay;
- no graphical dependency.

### LLM/language tests

For deterministic templates:

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
- traces are created or explicitly absent for causal reasons;
- witnesses have partial observations;
- belief provenance exists;
- institution records partial claims, not truth;
- replay rebuilds projections;
- TUI can inspect actor-known state;
- debug can inspect full causal graph.

## No-human simulation gate

No-human simulation is normal simulation with no human controller bound.

Once ordinary routines exist, no-human gates should run for at least several simulated days and produce meaningful ordinary-life events:

- meals;
- sleep;
- work;
- household movement;
- conversations;
- storage use;
- expectation checks;
- reports/notices if warranted;
- wrong beliefs or stale information where circumstances create them;
- planner failures/recoveries;
- no protagonist gravity.

A no-human run that produces only idle loops fails architecture validation.

## No-player privilege gate

For every world-affecting action exposed to TUI:

- an AI actor under equivalent conditions can propose it;
- the same action definition is used;
- the same preconditions are checked;
- the same events are committed;
- the same traces/observations/norm hooks apply;
- rejection/failure is equivalent.

Debug-only actions must be visibly non-diegetic and impossible in embodied play.

## Actor-knowledge gate

Run the same scenario from multiple possessions.

Example:

- possess Tomas before discovering missing coins;
- possess Mara after hiding coins;
- possess Elena after hearing noise;
- possess clerk after receiving report;
- inspect each notebook;
- verify no notebook contains another actor's private knowledge;
- verify debug view can show mismatch.

## Deterministic replay gate

Required checks:

- same seed/event log produces same projections;
- replay from snapshot equals replay from genesis for covered state;
- projection rebuild is deterministic;
- random stream positions match;
- event schema fixtures replay;
- upcaster tests cover old events;
- unknown event version fails loudly;
- content version mismatch fails or migrates explicitly;
- save-package manifests identify all required versions and checksums.

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

## Observability artifacts

Automated runs should produce artifacts useful for review:

- event log segment;
- save-package manifest;
- replay checksum report;
- projection rebuild report;
- causal graph excerpt;
- actor belief diffs;
- actor notebook snapshots;
- TUI transcript;
- why-not log;
- content validation report;
- scheduler queue/reservation snapshots;
- planner trace samples;
- institution procedure trace;
- LOD transition report;
- story-sifter debug summary;
- leakage audit;
- LLM validation report if applicable.

Artifacts must be inspectable without a graphical client.

## Metrics

Metrics are not game scores. They are review instruments.

Useful metric families:

```text
event_counts_by_type
routine_event_rate
significant_event_rate
action_rejection_rate_by_reason
action_failure_rate_by_type
belief_update_count
contradiction_count
trace_created_discovered_decayed_counts
reports_records_notices_counts
stale_artifact_count
planner_failure_count
stuck_actor_count
no_human_activity_coverage
replay_duration_and_checksum_status
projection_rebuild_duration
random_draws_by_stream
actor_knowledge_leakage_findings
player_conditioned_event_rate
```

Metrics must not drive hidden drama. They expose behavior for humans and tests to judge.

## Review report contract

A runnable phase should emit a review report like:

```yaml
RunReviewReport:
  run_id: no_human_village_day_003
  content_manifest: neutral_village@0.1
  seed: first_village_seed_07
  sim_interval: 142-08-12T05:30/142-08-15T05:30
  event_count: 4812
  significant_events: 143
  replay:
    authoritative: pass
    checksums: pass
  projections:
    rebuild: pass
  actor_knowledge:
    leakage_audit: pass
    notebooks_checked: [actor_tomas, actor_mara, actor_elena, actor_anna_clerk]
  no_player_privilege:
    action_registry_parity: pass
  issues:
    - actor_evan_guard_idle_18h_due_missing_patrol_method
  artifacts:
    - event_log_excerpt
    - causal_graph_missing_property
    - tui_transcript_tomas
    - planner_trace_mara
```

## Acceptance levels

### Architecture accepted

The feature has a boundary contract and does not violate doctrine.

### Prototype accepted

The feature runs in deterministic fixture tests and exposes debug inspection.

### First-slice accepted

The feature runs in no-human simulation, embodied TUI play, replay, projection rebuild, and actor-knowledge tests.

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
- Institution test asserts truth instead of institutional belief.
- Content validation stops at syntax.
- Debug tools cannot explain why an event happened.
- Metrics become a hidden pacing director.
