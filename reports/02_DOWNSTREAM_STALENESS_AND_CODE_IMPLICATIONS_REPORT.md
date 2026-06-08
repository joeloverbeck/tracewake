# Downstream Staleness and Code Implications Report

## Status

Replacement report. The architecture layer has now been reviewed and replaced as part of the architecture replacement mission. This report no longer treats `docs/1-architecture/**` as unchecked downstream staleness.

## Evidence basis

This report is based on exact target-commit reads of:

- `docs/README.md`;
- every file under `docs/0-foundation/**`;
- every file under the previous `docs/1-architecture/**` set;
- representative Rust seams for scheduler/no-human simulation, proposal/validation pipeline, actor-known context, decision transaction, candidates, intentions, routines, local planning, traces/stuck diagnostics, epistemics, event/replay, TUI view models, TUI command loop, fixtures, and acceptance tests;
- the uploaded manifest as path inventory only.

The target commit was user-supplied. This report does not claim the commit is current `main`.

## Architecture replacement outcome

The architecture layer has been replaced with a normalized contract set:

```text
docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
```

The previous architecture document that framed actor-known autonomy as a Phase 3A clarification has been absorbed and normalized into the architecture spine. Holder-known context and truth-firewall rules now appear early and constrain every planning/procedure/view/LOD subsystem. The previous state/content authoring concerns are preserved in the authority, ordinary-life, validation, and research documents rather than left as an isolated stale contract.

## Remaining downstream implications

### `docs/2-execution/**`

Execution documents may still contain old architecture filenames, phase-local phrasing, or weaker acceptance language. They should be realigned later to the replacement architecture map. Priority areas:

- truth-firewall and holder-known transaction gates;
- no-human ordinary-life proof requiring decision/context/stuck diagnostics;
- content validation for actor-known source tags;
- TUI embodied/debug test coverage as non-negotiable;
- institution-known procedure gates;
- LOD promotion ancestry before second-proof expansion.

This report does not create execution specs or tickets.

### `docs/3-reference/**`

Reference documents should be updated later to include or sharpen terms and risk checks for:

- holder-known context;
- truth firewall;
- actor-known and institution-known transaction;
- validation truth;
- actor-legible why-not;
- debug-only truth;
- provenance graph;
- typed decision trace;
- typed stuck diagnostic;
- LOD promotion ancestry;
- story-sifting projection;
- LLM-disabled acceptance.

### `docs/4-specs/**` and archived specs/tickets

Older specs and archived tickets are historical implementation material. They may contain useful examples and anti-regression history, but they are subordinate to foundation and the replacement architecture. Later corrective specs should cite the new architecture map instead of reviving phase-local shortcuts or stale filenames.

### Code hardening pressure

Current Rust implementation is partial. It includes promising seams, but it should not be treated as doctrine.

Observed implementation pressure:

- scheduler/no-human code has an autonomy path and no-human day reporting, but future hardening must continue proving that windows and needs trigger actor-known transactions rather than primitive scheduler actions;
- action pipeline code separates proposals, validation reports, actor-visible summaries, debug summaries, and event application, but future hardening should make actor/debug failure semantics impossible to blur;
- actor-known, decision, HTN/routine, intention, and trace code already points toward provenance-bearing decisions, but the architecture now requires this boundary structurally for every cognition/procedure subsystem, not by convention;
- decision traces and stuck diagnostics exist as typed structures, but tests and future reports should avoid treating display/rendered summaries as proof;
- epistemic projection code supports holder filtering, observations, beliefs, contradictions, and debug inspection; future work should extend the same discipline to institutions, speech, LOD, and leads;
- TUI code and tests exercise embodied view models, semantic action IDs, why-not output, debug panels, and command parsing; architecture now treats this as acceptance-critical, not optional UI polish;
- content fixtures and validation tests already include anti-contamination examples, but future content validation should reject missing provenance for records, routines, household knowledge, workplace/home/bed/food knowledge, notices, and prehistory;
- LOD/regional/institution procedure models are not yet complete and must not inherit assumptions from the partial ordinary-life implementation.

### Tests and fixtures

Remaining test pressure:

- add adversarial tests where true food/work/sleep/route/suspect/proof exists in authoritative state but is absent from holder-known context;
- assert context packets/provenance lists, not only hidden-truth audit strings;
- ensure no-human success requires ordinary action proposals, validation, events, replay, and diagnostics;
- prove institution-known procedures before any record/notice/sanction system can pass;
- prove TUI actor-legible why-not never contains debug truth;
- prove debug truth/belief comparisons are visibly non-diegetic and cannot feed embodied view models;
- prove LLM-disabled operation for all acceptance-critical language flows.

### Reports and future corrective specs

Future reports should no longer say the architecture layer has not been checked. If they find architecture drift after this replacement, they should identify the exact document and exact unresolved contract.

Future corrective specs should be narrow and subordinate. They should not re-litigate architecture. They should implement or harden the contracts now stated in `docs/1-architecture/**`.

## No unresolved architecture staleness claim

No broad unresolved architecture-staleness claim remains for `docs/1-architecture/**` after this replacement. Remaining risk is downstream alignment: execution docs, reference docs, specs, code, tests, and fixtures may lag behind the corrected architecture contract.

## Non-goals

This report does not provide implementation tickets, execution specs, code patches, AGENT tasks, or branch plans. It is a replacement downstream report aligned to the corrected architecture set.
