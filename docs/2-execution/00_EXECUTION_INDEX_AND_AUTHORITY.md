# Execution Index and Authority

## Status

This folder is the complete replacement set for `docs/2-execution/*` at the target commit supplied for this mission.

These documents define execution order, phase gates, fixture contracts, validation contracts, debug/observability expectations, data-authoring boundaries, and first-proof/second-proof limits. They do not define implementation tickets, sprint plans, Rust source code, final crate choices, or UI polish.

## Exact-commit source statement

I am not verifying that this commit is the current `main`. I am using your supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.

Target repository and commit:

```text
repository: joeloverbeck/tracewake
target commit: 5f01f72e0d3f42243becd95160a98cf7565fdb1c
```

Repository source handling for this replacement set:

- the uploaded manifest was used only as the path inventory;
- needed repository files were fetched only by the exact raw URLs listed in the evidence ledger; each uses the repository, commit, and path literally;
- no clone was used;
- no GitHub code search was used;
- no branch name, default-branch lookup, or repository metadata was used as authority;
- no connector namespace label was used as authority;
- no needed fetched repository path was outside the uploaded manifest;
- no fetched repository URL in the evidence ledger points to `joeloverbeck/one-more-branch` or to any repository other than `joeloverbeck/tracewake`.

## Evidence ledger: exact repository URLs fetched

| Manifest path | Exact URL fetched |
|---|---|
| `README.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/README.md` |
| `docs/0-foundation/00_FOUNDATION_INDEX.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/00_FOUNDATION_INDEX.md` |
| `docs/0-foundation/01_PROJECT_CHARTER.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/01_PROJECT_CHARTER.md` |
| `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` |
| `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` |
| `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` |
| `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` |
| `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` |
| `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` |
| `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` |
| `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` |
| `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` |
| `docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md` |
| `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` |
| `docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` |
| `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` |
| `docs/1-architecture/01_SYSTEM_AUTHORITY_RUST_WORKSPACE_AND_BOUNDARIES.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/01_SYSTEM_AUTHORITY_RUST_WORKSPACE_AND_BOUNDARIES.md` |
| `docs/1-architecture/02_ACTION_AFFORDANCE_SCHEDULING_AND_PROPOSAL_PIPELINE.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/02_ACTION_AFFORDANCE_SCHEDULING_AND_PROPOSAL_PIPELINE.md` |
| `docs/1-architecture/03_EVENT_LOG_REPLAY_PROJECTIONS_AND_SAVE_PACKAGES.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/03_EVENT_LOG_REPLAY_PROJECTIONS_AND_SAVE_PACKAGES.md` |
| `docs/1-architecture/04_STATE_CONTENT_AUTHORING_DOMAIN_PACKS_AND_SCHEMA_VALIDATION.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/04_STATE_CONTENT_AUTHORING_DOMAIN_PACKS_AND_SCHEMA_VALIDATION.md` |
| `docs/1-architecture/05_CLAIMS_BELIEFS_TRACES_MEMORY_AND_INFORMATION_FLOW.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/05_CLAIMS_BELIEFS_TRACES_MEMORY_AND_INFORMATION_FLOW.md` |
| `docs/1-architecture/06_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/06_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` |
| `docs/1-architecture/07_AGENT_COGNITION_ROUTINES_PLANNING_AND_AGENT_LOD.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/07_AGENT_COGNITION_ROUTINES_PLANNING_AND_AGENT_LOD.md` |
| `docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` |
| `docs/1-architecture/09_ORDINARY_LIFE_ECONOMY_SETTLEMENT_AND_SPATIAL_MODEL.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/09_ORDINARY_LIFE_ECONOMY_SETTLEMENT_AND_SPATIAL_MODEL.md` |
| `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` |
| `docs/1-architecture/11_INCIDENTS_LEADS_AND_STORY_SIFTING_PROJECTIONS.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/11_INCIDENTS_LEADS_AND_STORY_SIFTING_PROJECTIONS.md` |
| `docs/1-architecture/12_LOD_REGIONAL_PROCESSES_WORLD_HISTORY_AND_SCALE.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_WORLD_HISTORY_AND_SCALE.md` |
| `docs/1-architecture/13_VALIDATION_OBSERVABILITY_METRICS_AND_ACCEPTANCE_ARCHITECTURE.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/13_VALIDATION_OBSERVABILITY_METRICS_AND_ACCEPTANCE_ARCHITECTURE.md` |
| `docs/1-architecture/14_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/14_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` |
| `docs/3-reference/01_DESIGN_RISK_REGISTER.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/3-reference/01_DESIGN_RISK_REGISTER.md` |
| `docs/3-reference/02_GLOSSARY.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/3-reference/02_GLOSSARY.md` |
| `docs/README.md` | `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/README.md` |

## Uploaded execution source material analyzed

The uploaded current execution documents were treated as strong source material, not final truth:

- `01_EXECUTION_CHARTER_AND_SCOPE_DECISIONS.md`
- `02_STRICT_PHASE_LADDER_AND_IMPLEMENTATION_ORDER.md`
- `03_FIRST_PROOF_THE_MISSING_PROPERTY_VILLAGE.md`
- `04_PHASE_0_PAPER_ONTOLOGY_AND_FIXTURE_DESIGN.md`
- `05_PHASE_1_RUNNABLE_KERNEL_TUI_AND_EVENT_LOG.md`
- `06_PHASE_2_EPISTEMICS_VIEW_MODELS_AND_POSSESSION_PARITY.md`
- `07_PHASE_3_NEEDS_ROUTINES_AND_NO_HUMAN_DAILY_SIM.md`
- `08_PHASE_4_INSTITUTIONS_RECORDS_AND_WRONG_SUSPICION.md`
- `09_DATA_AUTHORING_SCHEMAS_AND_GOLDEN_FIXTURES.md`
- `10_TESTING_VALIDATION_DEBUGGING_AND_METRICS.md`
- `11_SECOND_PROOF_NOTICES_TRAVEL_AND_REGIONAL_EXPANSION.md`
- `12_RESEARCH_DECISIONS_AND_EXECUTION_SOURCE_NOTES.md`

The uploaded source-notes document contained a stale prior target commit and stale execution path inventory. This replacement set does not preserve that session metadata or the old path list.

## Authority relationship

Execution obeys this authority order:

```text
foundation doctrine
 -> architecture contracts
 -> execution phase gates and fixtures
 -> implementation specs/code
 -> tests and validation reports
```

When execution conflicts with foundation or architecture, execution is wrong. When code or tests make a convenient exception to an execution gate, the code or test is wrong.

## Replacement document map

Read in order unless working on a specific phase.

| File | Purpose |
|---|---|
| `00_EXECUTION_INDEX_AND_AUTHORITY.md` | Folder map, exact-commit evidence ledger, authority order, and anti-drift rules. |
| `01_EXECUTION_CHARTER_SCOPE_AND_FIRST_PROOF.md` | Execution identity, product priority, first-proof boundary, second-proof deferral, TUI/no-human/LLM policies. |
| `02_PHASE_LADDER_AND_ACCEPTANCE_GATES.md` | Compact strict phase ladder, universal phase gates, and phase exit table. |
| `03_FIRST_PROOF_MISSING_EXPECTED_PROPERTY.md` | Missing Expected Property / Missing Property Village proof contract, roster, map scope, scenarios, and definition of done. |
| `04_PHASE_0_PAPER_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` | Paper ontology severity, action/event/proposition/trace/speech-act vocabulary, fixture chains, and view-model contracts. |
| `05_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY.md` | Runnable physical/event/TUI spine, action pipeline, minimal fixture, event log, replay, and no-human scheduler gate. |
| `06_PHASE_2_EPISTEMICS_VIEW_MODELS_AND_POSSESSION_PARITY.md` | Observation, belief, expectation contradiction, actor-known filtering, debug view, and no-knowledge-transfer proof. |
| `07_PHASE_3_NEEDS_ROUTINES_AND_NO_HUMAN_LIFE.md` | Hunger, fatigue, safety, sleep, food, work, routines, interruptions, and no-human day. |
| `08_PHASE_4_INSTITUTIONS_RECORDS_AND_WRONG_SUSPICION.md` | Households, local authority, reports, records, norms, procedure, and wrong suspicion. |
| `09_DATA_AUTHORING_VALIDATION_AND_GOLDEN_FIXTURES.md` | Logical schema contracts, validation pipeline, no-script rules, and golden fixture definitions. |
| `10_TESTING_DEBUGGING_OBSERVABILITY_AND_METRICS.md` | Testing matrix, golden scenarios, TUI/view-model tests, replay/debug expectations, metrics, and failure doctrine. |
| `11_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_AND_REGIONAL_EXPANSION.md` | Second-proof notices, stale leads, travel, companions, proof/payment, and regional expansion boundaries. |
| `12_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` | Corrected source handling, research decisions, and design consequences. |

## Product guardrails preserved

Tracewake is a causality-first, epistemic, ordinary-life simulation engine and playable TUI-first simulation. Execution must preserve:

- causality before drama;
- belief before truth;
- ordinary life before adventure;
- no sacred player entity;
- every world-affecting player action must be possible for an ordinary agent under equivalent conditions;
- institutions are fallible social machines;
- quests are projections, not ontology;
- authored causal machinery is allowed;
- authored outcome chains are forbidden;
- symbolic, inspectable agents before generative agents;
- TUI-first, playable always;
- genre-agnostic kernel;
- story is observed, not directed;
- LLMs may render or parse language only behind validation;
- LLMs are not authoritative simulation brains;
- Rust-first implementation;
- event sourcing and forensic causality are foundational;
- no-human simulation is mandatory in runnable phases;
- UI is not a rules engine;
- persistence is not the simulation model;
- domain packs cannot bypass the action/event pipeline;
- debug tools cannot leak into embodied play.

## Universal execution questions

Every accepted mechanic, fixture, screen, data contract, and later implementation spec must answer:

```text
What caused it?
Who knows it?
How can they be wrong?
What traces exist?
What institution, household, norm, role, record, or public artifact cares?
Can an NPC do the same kind of thing?
Can it be played or reached through TUI/view models?
Does it leak player privilege, ground truth, genre assumptions, scripting, or LLM authority?
Can it run in no-human simulation?
Can debug explain it?
Can replay rebuild it?
```

## Hard failure patterns

Reject any execution or implementation path that introduces:

- `Quest`, `Objective`, `PlayerCharacter`, `Reward.spawn_on_completion`, or `true_culprit_for_institution` as core ontology;
- direct inventory or record mutation from data scripts during ordinary play;
- an embodied TUI built from hidden truth and manually hidden fields;
- institutional suspicion or record creation from ground truth;
- no-human simulation postponed until after ordinary life or institutions;
- LLM output as fact, plan, proof, record, clue, guilt, or state mutation;
- a notice/travel/companion loop before the missing-property proof passes;
- debug commands substituting for embodied play.

## First execution standard

Build a small, TUI-playable, replayable ordinary village whose people can be wrong for the right reasons. The first proof is missing expected property, not an adventure quest. The second proof begins only after the village can explain expectation, absence, partial observation, report, record, wrong suspicion, possession parity, no-human life, and replay.
