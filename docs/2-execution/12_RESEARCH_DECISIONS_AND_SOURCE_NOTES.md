# Research Decisions and Source Notes

## Status

This document records execution-specific source handling and research decisions for the replacement execution set. It is not a literature review, implementation ticket list, migration note, or session transcript.

The old uploaded source-notes document was not preserved because it contained a stale target commit, stale path inventory, and session-specific connector-failure history. This replacement keeps only stable source-handling policy and the design consequences that should guide execution.

## Exact-commit repository handling

I am not verifying that this commit is the current `main`. I am using your supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.

Target repository and commit:

```text
repository: joeloverbeck/tracewake
target commit: 5f01f72e0d3f42243becd95160a98cf7565fdb1c
```

Rules followed for repository sources:

- the uploaded manifest was used only as the path inventory;
- every needed repository source path was present in the uploaded manifest before being fetched;
- repository content was fetched only by the exact raw URLs listed below; each uses the repository, commit, and path literally;
- no branch name, default branch, repository metadata lookup, GitHub code search, clone, or connector namespace label was used as repository authority;
- uploaded current execution docs were analyzed as uploaded artifacts, not fetched through repository metadata;
- no migration notes file is part of this replacement set.

## Evidence ledger: exact repository URLs fetched

- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/README.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/00_FOUNDATION_INDEX.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/01_PROJECT_CHARTER.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/01_SYSTEM_AUTHORITY_RUST_WORKSPACE_AND_BOUNDARIES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/02_ACTION_AFFORDANCE_SCHEDULING_AND_PROPOSAL_PIPELINE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/03_EVENT_LOG_REPLAY_PROJECTIONS_AND_SAVE_PACKAGES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/04_STATE_CONTENT_AUTHORING_DOMAIN_PACKS_AND_SCHEMA_VALIDATION.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/05_CLAIMS_BELIEFS_TRACES_MEMORY_AND_INFORMATION_FLOW.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/06_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/07_AGENT_COGNITION_ROUTINES_PLANNING_AND_AGENT_LOD.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/09_ORDINARY_LIFE_ECONOMY_SETTLEMENT_AND_SPATIAL_MODEL.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/11_INCIDENTS_LEADS_AND_STORY_SIFTING_PROJECTIONS.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_WORLD_HISTORY_AND_SCALE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/13_VALIDATION_OBSERVABILITY_METRICS_AND_ACCEPTANCE_ARCHITECTURE.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/1-architecture/14_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/3-reference/02_GLOSSARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/tracewake/5f01f72e0d3f42243becd95160a98cf7565fdb1c/docs/README.md`

## Internal source authority used

Authority order:

```text
foundation doctrine
 -> architecture contracts
 -> execution gates and fixture contracts
 -> future implementation specs/code
 -> tests and validation reports
```

Foundation documents used as constitutional authority:

- `docs/0-foundation/00_FOUNDATION_INDEX.md`
- `docs/0-foundation/01_PROJECT_CHARTER.md`
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`
- `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md`
- `docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md`
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`
- `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`
- `docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md`
- `docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md`
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md`

Architecture documents used as contract authority:

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `docs/1-architecture/01_SYSTEM_AUTHORITY_RUST_WORKSPACE_AND_BOUNDARIES.md`
- `docs/1-architecture/02_ACTION_AFFORDANCE_SCHEDULING_AND_PROPOSAL_PIPELINE.md`
- `docs/1-architecture/03_EVENT_LOG_REPLAY_PROJECTIONS_AND_SAVE_PACKAGES.md`
- `docs/1-architecture/04_STATE_CONTENT_AUTHORING_DOMAIN_PACKS_AND_SCHEMA_VALIDATION.md`
- `docs/1-architecture/05_CLAIMS_BELIEFS_TRACES_MEMORY_AND_INFORMATION_FLOW.md`
- `docs/1-architecture/06_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md`
- `docs/1-architecture/07_AGENT_COGNITION_ROUTINES_PLANNING_AND_AGENT_LOD.md`
- `docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md`
- `docs/1-architecture/09_ORDINARY_LIFE_ECONOMY_SETTLEMENT_AND_SPATIAL_MODEL.md`
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `docs/1-architecture/11_INCIDENTS_LEADS_AND_STORY_SIFTING_PROJECTIONS.md`
- `docs/1-architecture/12_LOD_REGIONAL_PROCESSES_WORLD_HISTORY_AND_SCALE.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_METRICS_AND_ACCEPTANCE_ARCHITECTURE.md`
- `docs/1-architecture/14_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md`

Reference/context documents used for terminology, risk alignment, and documentation context:

- `README.md`
- `docs/README.md`
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md`
- `docs/3-reference/02_GLOSSARY.md`

Uploaded execution source material replaced:

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

## Correction of stale source-note material

The uploaded source-notes document referenced a previous target commit and old architecture/execution path names. This replacement corrects those errors by:

- using only `5f01f72e0d3f42243becd95160a98cf7565fdb1c` as the target commit;
- listing the actual manifest paths at that commit;
- removing session-specific connector failure stories;
- not asserting that `5f01f72e0d3f42243becd95160a98cf7565fdb1c` is current `main`;
- not preserving old execution paths such as prior `18_` through `22_` execution documents;
- converting source handling into stable policy rather than session narrative.

## Why the first proof is narrowed

Execution decision:

```text
First proof: Missing Expected Property / The Missing Property Village.
Second proof: Notices, Travel, and Regional Expansion.
```

Reason:

A missing expected property exercises Tracewake's doctrine more directly than a road-threat expedition. It requires homes, rooms, doors, containers, ownership, custody, expected location, needs, routines, observation, fallible testimony, belief provenance, report intake, records, household and local authority behavior, wrong suspicion, possession parity, no-human life, TUI play, replay, and debug explanation.

Road-threat/travel remains important, but it can hide weak fundamentals. A notice board can look like progress while records are omniscient. A companion can look alive while ordinary actors are props. A route expedition can feel like content while homes, work, access, expectation contradiction, and belief provenance are shallow.

Execution rule:

```text
Explain one missing expected item before explaining a road.
```

## Why the phase ladder is strict

Execution decision:

```text
Phase 0 paper ontology
 -> Phase 1 physical/action/event/TUI/replay spine
 -> Phase 2 epistemics/view/possession parity
 -> Phase 3 needs/routines/no-human life
 -> Phase 4 institutions/records/wrong suspicion
 -> second proof only afterward
```

Reason:

Architecture requires a shared action pipeline, event log, replay, actor-known filtering, TUI/view-model boundary, symbolic agent cognition, and fallible institutions. Building planning, social authority, travel, or public leads before those contracts exist invites parallel rule systems and hidden scripts.

Execution rule:

```text
later phase excitement cannot backfill earlier phase weakness
```

## Event sourcing, CQRS, and replay

Research influence:

Event sourcing preserves a sequence of state-changing events and supports reconstructing state from those events. CQRS-style separation reinforces the difference between authoritative writes/events and derived read models/projections.

Design consequence for Tracewake:

```text
Phase 1 must include event log, event envelope, deterministic ordering, replay,
projection rebuild, and debug explanation for item location.
```

Tracewake needs this because the first proof asks forensic questions a current-state save cannot answer:

- who moved the item?
- why was the action possible?
- what traces did it leave?
- who observed it?
- which beliefs and records derive from it?
- why did wrong suspicion arise?
- did embodied TUI leak truth?
- can no-human simulation be audited?

Execution rule:

```text
current state answers what is true now
event log answers how it became true
actor/institution projections answer who believes what and why
```

## Deterministic simulation and randomness

Research influence:

Deterministic replay and lockstep-style thinking require stable inputs, stable ordering, isolated randomness, versioned content, and avoidance of wall-clock or platform accidents.

Design consequence:

```text
no wall-clock authority
no unordered iteration authority
no hidden random authority
no live network or LLM timing authority in simulation decisions
meaningful random draws are purpose-labeled and replay-auditable
```

Phase 1 establishes discrete time and stable ordering. Phase 3 expands scheduler pressure through sleep, eating, work, waiting, routine interruption, and no-human simulation.

## BDI, HTN, GOAP, utility, and bounded planning

Research influence:

BDI separates beliefs, desires/motives, and intentions. HTN methods are useful for explainable routines and procedures. GOAP-style local planning is useful for finding concrete action sequences under constraints. Utility can rank explicit options, but it cannot become a magic mind.

Design consequence:

```text
agent cognition proposes
kernel validates
event log commits
debug explains
```

Planning before the action/event spine creates opaque behavior. The simulation must first prove actions, failures, events, traces, and replay. Then beliefs can constrain what agents know. Then routines can propose actions through the same pipeline.

GOAP is not the whole mind. HTN methods are not plot scripts. Utility scores are not causality.

## Social simulation and emergent narrative precedents

Research influence:

Talk of the Town, Prom Week, Comme il Faut, and Neighborly show that social facts, knowledge, relationships, history, statuses, and rules can generate emergent narrative without hand-authoring every outcome.

Design consequence:

Tracewake uses social machinery, but with stricter forensic requirements:

```text
relationship/social information must have causal source and future effect
social facts cannot be hidden quest flags
beliefs, records, speech, and suspicions need provenance
```

The first proof therefore emphasizes source-backed beliefs, partial witness observations, speech acts, institutional records, and wrong suspicion instead of a puzzle solution.

## Normative multi-agent systems

Research influence:

Normative MAS work treats norms as explicit structures that guide, regulate, detect, and respond to behavior in multi-agent settings. Norms can be violated, detected, revised, enforced, ignored, or misunderstood.

Design consequence:

Tracewake institutions and households must distinguish:

```text
violation
detection
suspicion
report
record
proof
sanction
```

Execution rule:

```text
institution acts from institutional knowledge and procedure, not truth
```

A record exists because an actor or institution created an artifact through a modeled channel. It does not exist because a violation happened in ground truth.

## Dynamic level of detail and scale

Research influence:

Dynamic LOD research highlights the tradeoff between detail and scale and the need to adapt fidelity deliberately.

Design consequence:

The first proof starts small:

```text
10-20 high-detail named agents initially
expand toward 10-30 only after stability
low-LOD background only if identity-bearing, causally honest, and promotable
```

Execution rule:

```text
scale only after replay, no-human simulation, actor-known filtering, records, and debug explanation remain strong
```

A large village that cannot explain one missing property is hollow.

## LLM social simulation and language boundary

Research influence:

Generative-agent research shows that LLM-backed agents can produce compelling social surfaces, memory summaries, reflection, and conversation. LLM-agent simulation surveys and boundary critiques also highlight reliability, bias, behavioral heterogeneity, reproducibility, interpretability, and validation risks.

Design consequence:

LLMs are deferred beyond the first proof. First-proof language uses deterministic templates or mocks.

Execution rule:

```text
structured speech act is authority
surface prose is rendering
LLM output is optional and validated
core works with LLM disabled
```

LLMs may not decide actions, parse authoritative freeform play, create facts, create records, create clues, determine guilt, mutate state, create quests, or make golden tests depend on live output.

## Game and simulation precedents converted into execution rules

### Dwarf Fortress

Decision:

Deep world history and procedural consequence are inspiring, but world generation is not the first target.

Execution rule:

```text
hand-authored causal village before procedural history
```

### Shadows of Doubt

Decision:

Citizens with homes, jobs, routines, and evidence-rich spaces are a useful precedent. Tracewake narrows first to TUI ordinary-life epistemics rather than graphical city-scale detective play.

Execution rule:

```text
citizen independence first, city scale later
```

### The Sims smart-object lineage

Decision:

Object affordances are powerful for ordinary behavior. Tracewake uses typed affordances but routes every action through validation, events, beliefs, norms, traces, and replay.

Execution rule:

```text
object exposes action possibility
kernel decides validity
event log records consequence
```

### Ultima-style schedules

Decision:

Schedules make worlds feel alive, but Tracewake schedules must be defeasible intentions, not teleports.

Execution rule:

```text
routine = belief/need/role-driven method with failure modes
```

### RimWorld storytellers and Left 4 Dead AI Director

Decision:

Pacing directors are useful in their own games but are counterexamples for Tracewake's first proof.

Execution rule:

```text
no hidden drama director
story is observed after events, not directed before them
```

### Skyrim radiant quests

Decision:

Radiant quest systems scale tasks but preserve objective quest ontology. Tracewake uses notices, records, requests, contracts, suspicions, obligations, and leads as fallible artifacts/projections.

Execution rule:

```text
no quest object
no objective completion
no guaranteed reward
```

## Data format posture

Execution decision:

```text
logical contracts and validation first
final file syntax later
```

Candidate formats such as RON, TOML, YAML, JSON, or custom formats may be evaluated later. The doctrine is stable IDs, deterministic loading order, schema validation, referential integrity, versioned fixtures, source-backed beliefs, source-backed records/traces/speech acts, no outcome chains, no player schema, no quest ontology, and replay content-version compatibility.

YAML-like examples in these docs are logical contracts, not final syntax.

## Rust and tooling posture

Tracewake is Rust-first for the authoritative simulation core, event/replay machinery, action validation, actor-knowledge filtering, agent decision core, no-human simulation harness, and first TUI client.

Candidate ecosystem tools may include a terminal UI library, terminal backend, serialization/schema tooling, property-test tooling, snapshot/golden-test tooling, test runner, benchmark tooling, and tracing/logging/debug tooling. Candidate names are not doctrine.

The doctrine is:

```text
deterministic core
event sourcing
replay
schema validation
actor-knowledge filtering
action parity
no-human simulation
TUI/view-model reachability
clear authority boundaries
```

## External research and precedent sources consulted

These sources sharpened execution rules; they are not authority over the foundation or architecture:

- Event sourcing: `https://martinfowler.com/eaaDev/EventSourcing.html`
- Event sourcing pattern: `https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing`
- CQRS pattern: `https://learn.microsoft.com/en-us/azure/architecture/patterns/cqrs`
- Deterministic lockstep: `https://gafferongames.com/post/deterministic_lockstep/`
- Floating point determinism: `https://gafferongames.com/post/floating_point_determinism/`
- BDI agents: `https://cdn.aaai.org/ICMAS/1995/ICMAS95-042.pdf`
- BDI model of agency: `https://link.springer.com/chapter/10.1007/3-540-49057-4_1`
- HTN planning: `https://www.cs.umd.edu/~nau/papers/erol1994umcp.pdf`
- GOAP / F.E.A.R.: `https://www.gamedevs.org/uploads/three-states-plan-ai-of-fear.pdf`
- Talk of the Town knowledge: `https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter37_Simulating_Character_Knowledge_Phenomena_in_Talk_of_the_Town.pdf`
- Prom Week social physics: `https://www.ben-samuel.com/wp-content/uploads/2015/09/FDG-2011-Prom-Week-Social-Physics-as-Gameplay.pdf`
- Comme il Faut: `https://cdn.aaai.org/ojs/12454/12454-52-15982-1-2-20201228.pdf`
- Neighborly paper: `https://www.kmjn.org/publications/Neighborly_CoG22.pdf`
- Normative MAS introduction: `https://www.di.unito.it/~guido/PS/cmot06.pdf`
- Norms review: `https://pmc.ncbi.nlm.nih.gov/articles/PMC4119705/`
- Dynamic LOD: `https://www.ifaamas.org/Proceedings/aamas2011/papers/C5_B67.pdf`
- Multi-level agent-based simulation patterns: `https://hal.science/hal-01691388/document`
- Generative agents: `https://arxiv.org/abs/2304.03442`
- LLM agent-based modeling survey: `https://www.nature.com/articles/s41599-024-03611-3`
- LLM social simulations boundary: `https://openreview.net/pdf/0b3d19b16e1535f9d085a395aaa336696a8e3d82.pdf`
- Dwarf Fortress development: `https://www.bay12games.com/dwarves/dev.html`
- Dwarf Fortress world generation reference: `https://dwarffortresswiki.org/index.php/World_generation`
- Shadows of Doubt city simulation devblog: `https://colepowered.com/shadows-of-doubt-devblog-8-simulating-a-city/`
- Shadows of Doubt citizen simulation devblog: `https://colepowered.itch.io/shadows/devlog/78044/shadows-of-doubt-devblog-15-moving-in-the-citizens`
- Left 4 Dead AI systems: `https://steamcdn-a.akamaihd.net/apps/valve/2009/ai_systems_of_l4d_mike_booth.pdf`
- Skyrim radiant quests: `https://www.wired.com/2011/11/skyrim-infinite-quests/`
- RimWorld storytellers: `https://rimworldwiki.com/wiki/AI_Storytellers`
- Ultima-style NPC schedules discussion: `https://lycaeum.ultimacodex.com/npc-schedules/comment-page-1/`

## Final execution conclusion

The replacement execution set is intentionally strict. The first deliverable is not a dramatic village. It is a village that can be audited.

The correct order is:

```text
paper ontology
 -> physical/event/TUI spine
 -> epistemics/view/possession parity
 -> ordinary needs/routines/no-human day
 -> local authority/report/record/wrong suspicion
 -> second proof notices/travel/regional expansion
```

Any implementation path that reaches road-threat adventure before the village can explain missing expected property, wrong belief, fallible record, no-human routine, possession parity, and replay is taking the wrong path.
