# Research Decisions and Execution Source Notes

## Status

This document records execution-specific research decisions and source handling for the replacement execution set. It is not a literature review. It explains why the first proof is narrowed, why the phase gates are severe, why TUI and no-human simulation are mandatory early, why event sourcing/replay are tested from the start, why LLMs are deferred, and why data format remains flexible.

## Repository source handling

Target repository and commit used for replacement analysis:

```text
repository: joeloverbeck/tracewake
target commit: 9c51ccb83405686e41261e8297126c5733df7df9
uploaded manifest: manifest_2026-06-05.txt
```

Repository metadata available through the installed GitHub repository listing identified `joeloverbeck/tracewake` with default branch `main`. The connector's direct branch/SHA endpoints were scoped inconsistently in this session and returned data for a different repository namespace, so the current `main` branch SHA could not be independently confirmed through that endpoint. All repository file contents used for analysis were therefore fetched by exact GitHub blob URL pinned to commit `9c51ccb83405686e41261e8297126c5733df7df9` and cross-checked against the uploaded tree manifest.

No GitHub code search or snippet-based repository search was used.

Target-fetched authority/source paths included:

```text
docs/0-foundation/*
docs/1-architecture/*
docs/2-execution/18_LIFE_POSSESSION_VERTICAL_SLICE_SPEC.md
docs/2-execution/19_ENGINEERING_ROADMAP_TUI_FIRST.md
docs/2-execution/20_TESTING_VALIDATION_AND_DEBUGGING.md
docs/2-execution/21_AUTHORING_GUIDE.md
docs/2-execution/22_STARTER_DATA_SCHEMAS.md
```

The replacement set treats:

- `docs/0-foundation/*` as constitutional authority;
- `docs/1-architecture/*` as architecture authority;
- old `docs/2-execution/*` as strong prior thinking and source material, not final truth.

## Why the first proof is narrowed

Execution decision:

```text
The first proof is The Missing Property Village.
Road-threat, bounty, companion, travel, stale lead, and proof/payment are second proof.
```

Reason:

A missing-property village exercises the core doctrine more directly than a road-threat expedition. It requires ordinary life, property expectations, rooms, doors, containers, needs, routines, observation, fallible memory, speech, report, record, local authority, wrong suspicion, possession parity, TUI play, no-human simulation, replay, and debug explanation.

Road-threat/travel is valuable, but it can hide weak fundamentals. A notice board can look like progress while records are omniscient. A companion can look alive while ordinary actors are props. A route expedition can feel like content while homes, work, and belief provenance are shallow. The first proof must not be hijacked by adventure grammar.

Supporting internal authorities:

- foundation project charter: playable TUI village first, engine second, graphics later;
- foundation first playable scope: neutral ordinary village, not combat/travel/quest first;
- architecture ordinary-life model: road threat matters later; theft matters now because homes, locks, storage, expectations, records, neighbors, and authority exist;
- architecture questless leads: road-threat/bounty is second proof and not quest ontology.

## Why population starts small

Execution decision:

```text
Start with 10–20 high-detail named agents.
Expand toward 10–30 only after stability.
Allow low-LOD background only if promotable and causally honest.
```

Reason:

A small population allows every important action, belief, relationship, routine, record, and trace to be inspected. The first proof needs density, not volume. A village of 60 weak actors will conceal epistemic leakage, shallow routines, and record omniscience. A village of 12–18 well-modeled actors can expose those failures.

Research influence:

Dynamic level-of-detail literature supports explicit fidelity management for large simulations, but the architecture conclusion is to stage LOD after a hand-authored causal village proves ontology. Relevant sources include Navarro et al., "Dynamic Level of Detail for Large Scale Agent-Based Urban Simulations" and broader multi-level agent-based simulation literature.

Execution rule:

```text
Scale only after replay, no-human simulation, and debug explanation remain strong.
```

## Why TUI gates exist every phase

Execution decision:

```text
Every runnable phase must be TUI-playable or view-model-acceptance-testable.
```

Reason:

Tracewake is TUI-first. The TUI is not a later presentation layer; it is the first proof that the simulation can be embodied, inspected, and played without graphical distraction. A mechanic that works only through debug commands or headless tests is not first-proof ready.

The TUI gate also protects actor-knowledge filtering. If embodied view models are tested from the start, hidden truth leaks are caught before they become UI architecture.

Execution rule:

```text
headless mechanic + no TUI/view-model gate = not accepted
```

## Why no-human simulation starts in Phase 1

Execution decision:

```text
Every runnable phase includes a no-human gate.
```

Reason:

Tracewake has no sacred player entity. No-human simulation proves that the scheduler, action pipeline, event log, agents, institutions, and records do not require a protagonist. In Phase 1, no-human may only prove controller-free scheduler advance and no player references. By Phase 3, it must prove a real ordinary day.

Execution rule:

```text
no-human simulation is normal simulation with no controller bound
```

If a system needs a human to begin or validate its world logic, it violates the premise.

## Why event sourcing and projection rebuild are tested early

Execution decision:

```text
Phase 1 includes event log, deterministic ordering, replay, and item-location explanation.
```

Research influence:

Event sourcing sources such as Martin Fowler's "Event Sourcing" and the Microsoft Azure Architecture Center's Event Sourcing and CQRS pattern notes frame the practical reason: event history explains how state came to be, while projections serve read/view needs. Tracewake needs this not for enterprise fashion but for forensic simulation.

Execution reason:

The first proof asks questions that a mutable save file cannot answer reliably:

- who moved the item?
- why was it possible?
- what traces came from it?
- who observed it?
- which beliefs and records derive from it?
- why is suspicion wrong?
- did embodied TUI leak truth?
- can no-human simulation be audited?

Projection rebuild is tested early because TUI, debug, actor notebooks, institution records, and story-sifter summaries are derived views. Derived views must not become authority.

Execution rule:

```text
current state answers what is true now
event log answers how it became true
actor/institution projections answer who believes what and why
```

## Why deterministic scheduling and randomness are phased early

Execution decision:

```text
Phase 1 establishes discrete time, stable ordering, and replay discipline.
Phase 3 expands scheduler use through sleep, work, waiting, and routines.
```

Research influence:

Deterministic lockstep and replay engineering literature, including Glenn Fiedler's deterministic simulation writing and floating-point determinism discussions, supports the practical rule that replay requires stable inputs, order, seeds, data versions, and isolation from wall-clock accident.

Execution rule:

```text
no wall-clock authority
no unordered iteration authority
no live network/LLM timing authority
meaningful random draws are purpose-labeled and auditable
```

## Why BDI/HTN/bounded planning is phased after the physical/event spine

Execution decision:

```text
Phase 1 builds action/event/TUI spine.
Phase 2 builds epistemics.
Phase 3 builds needs/routines/HTN-like ordinary life.
```

Research influence:

BDI research emphasizes separation of beliefs, desires/motives, and intentions. HTN planning supports explainable routine/procedure decomposition. GOAP-style planning, as used in game AI practice, is useful for bounded local action sequences. None of these should bypass action validation or mutate state directly.

Execution reason:

Planning before the action/event spine produces opaque behavior. The simulation must first prove that actions, failures, events, traces, and replay exist. Then beliefs can constrain what agents know. Then routines can propose actions through the same pipeline.

Execution rule:

```text
agent cognition proposes
kernel validates
event log commits
debug explains
```

GOAP is not the whole mind. HTN methods are not plot scripts. Utility scores are not causality.

## Why LLMs are deferred

Execution decision:

```text
LLMs are deferred beyond the first proof.
First proof uses deterministic templates/mocks only.
```

Research influence:

Generative Agents shows that LLMs can produce compelling social surfaces and memory-style summaries. Recent LLM social-simulation work also highlights validation, bias, behavioral consistency, heterogeneity, reproducibility, and interpretability risks. Relevant sources include:

- Park et al., "Generative Agents: Interactive Simulacra of Human Behavior";
- Taillandier et al., "Integrating LLM in Agent-Based Social Simulation: Opportunities and Challenges";
- Wu et al., "LLM-Based Social Simulations Require a Boundary";
- Zeng et al., "Too Human to Model";
- Bian et al., "Social Simulations with Large Language Model Risk Utopian Illusion."

Execution reason:

Tracewake needs deterministic replay, actor-knowledge filtering, causal inspection, reproducible tests, and institutional fallibility. LLMs are poor authoritative brains for those requirements. They may later render or parse structured language behind validation, but they cannot decide truth, action, guilt, records, clues, or state mutation.

Execution rule:

```text
structured speech act is authority
surface prose is rendering
LLM output is optional and validated
core works with LLM disabled
```

## Why data format remains flexible

Execution decision:

```text
Define logical data contracts and validation rules.
Do not freeze RON/TOML/YAML/JSON/custom syntax yet.
```

Reason:

The first proof must discover the ergonomic shape of actions, routines, beliefs, traces, records, and fixtures. Freezing syntax before validation needs are understood creates format cargo culting. The architecture cares more about stable IDs, versioning, source-backed beliefs, deterministic loading, migration, and no-script enforcement.

Execution rule:

```text
schema and validation first
syntax after content pressure proves needs
```

Candidate Rust ecosystem tools such as serde and schema validators may help, but no crate is doctrine.

## Why failure cases are deliverables

Execution decision:

```text
Each phase must define modeled failure cases and tests.
```

Reason:

Success-only simulation is usually scripted. A causal system must reject actions, fail actions, misperceive, delay, refuse, lose confidence, misrecord, form wrong suspicion, and replay mismatches. Failure proves that preconditions, knowledge, norms, institutions, and time are real.

Execution rule:

```text
no modeled failure = no accepted mechanic
```

## Why road-threat/travel is second proof

Execution decision:

```text
Second proof: Notices, Travel, and Regional Expansion.
```

Reason:

A route threat depends on first-proof machinery:

- reports need speech and records;
- notices need authorship, issuer, source claims, and physical artifacts;
- stale leads need belief/source/time;
- companions need ordinary actor needs/relationships;
- travel needs scheduler, route events, fatigue/safety, and no teleportation;
- proof/payment needs records, funds/custody, and procedure;
- public belief must diverge from route truth.

Without first-proof completion, road-threat flow degenerates into a quest board.

Execution rule:

```text
notice = source-bound artifact
lead = actor-known projection
travel = event chain
proof = interpreted claim/evidence
payment = custody/procedure
never quest ontology
```

## Research precedents converted into execution rules

### Talk of the Town, Prom Week, Comme il Faut, Neighborly

Decision:

Social facts, relationships, knowledge, and history can drive emergent narrative. Tracewake should use social machinery, but with stricter event sourcing, TUI playability, and forensic causality.

Execution rule:

```text
relationship/social information must have causal source and future effect
```

### Normative multi-agent systems

Decision:

Institutions and norms should be explicit, not hidden flags. Violation, detection, suspicion, record, proof, and sanction are separate.

Execution rule:

```text
institution acts from institutional knowledge and procedure, not truth
```

### Dwarf Fortress

Decision:

Deep world history and procedural consequence are inspiring, but worldgen is not the first implementation target.

Execution rule:

```text
hand-authored causal village before procedural history
```

### Shadows of Doubt

Decision:

Citizens with homes, jobs, routines, and inspectable evidence are a useful precedent. Tracewake narrows first to TUI ordinary-life epistemics rather than graphical city-scale detective play.

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

Pacing directors work for those games but are explicit counterexamples for Tracewake.

Execution rule:

```text
no hidden drama director
story is observed after events, not directed before them
```

### Skyrim radiant quests

Decision:

Radiant quest systems scale tasks but preserve objective quest ontology. Tracewake uses notices, records, requests, contracts, and leads as fallible artifacts/projections.

Execution rule:

```text
no quest object
no objective completion
no guaranteed reward
```

## Execution-specific source list

Primary internal authority:

```text
docs/0-foundation/01_PROJECT_CHARTER.md
docs/0-foundation/02_FOUNDATIONAL_INVARIANTS.md
docs/0-foundation/03_CAUSAL_SIMULATION_CONTRACT.md
docs/0-foundation/04_EPISTEMIC_MODEL_AND_INFORMATION_FLOW.md
docs/0-foundation/05_AGENT_INTENTION_AND_PLANNING_DOCTRINE.md
docs/0-foundation/06_INSTITUTIONS_NORMS_HOUSEHOLDS_AND_RECORDS.md
docs/0-foundation/07_TUI_FIRST_PLAYABILITY_CONTRACT.md
docs/0-foundation/08_NO_SCRIPTING_AND_CAUSAL_AUTHORING_POLICY.md
docs/0-foundation/09_SCALE_LOD_LONG_SIMULATION_AND_REGIONAL_PROCESSES.md
docs/0-foundation/10_DOMAIN_BOUNDARY_AND_FIRST_PLAYABLE_SCOPE.md
docs/0-foundation/11_VALIDATION_REPLAY_AND_ACCEPTANCE_GATES.md
docs/0-foundation/12_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md

docs/1-architecture/01_ARCHITECTURE_CHARTER_AND_REPLACEMENT_DECISIONS.md
docs/1-architecture/02_RUST_WORKSPACE_AND_SYSTEM_BOUNDARIES.md
docs/1-architecture/03_COMMAND_ACTION_AND_AFFORDANCE_PIPELINE.md
docs/1-architecture/04_EVENT_SOURCING_CAUSAL_GRAPH_AND_PROJECTIONS.md
docs/1-architecture/05_STATE_MODEL_ENTITIES_COMPONENTS_AND_CONTENT_DATA.md
docs/1-architecture/06_TIME_SCHEDULING_REPLAY_AND_RANDOMNESS.md
docs/1-architecture/07_EPISTEMIC_INFORMATION_TRACE_AND_MEMORY_ARCHITECTURE.md
docs/1-architecture/08_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
docs/1-architecture/09_AGENT_COGNITION_PLANNING_AND_ROUTINES.md
docs/1-architecture/10_INSTITUTIONS_NORMS_HOUSEHOLDS_AND_RECORDS.md
docs/1-architecture/11_ORDINARY_LIFE_ECONOMY_SETTLEMENT_AND_SPATIAL_MODEL.md
docs/1-architecture/12_PLAYER_POSSESSION_VIEW_MODELS_TUI_AND_DEBUG.md
docs/1-architecture/13_QUESTLESS_LEADS_NOTICES_AND_STORY_SIFTING.md
docs/1-architecture/14_LOD_REGIONAL_PROCESSES_WORLD_HISTORY_AND_SCALE.md
docs/1-architecture/15_VALIDATION_TESTING_AND_ACCEPTANCE_ARCHITECTURE.md
docs/1-architecture/16_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
```

Old execution source material replaced:

```text
docs/2-execution/18_LIFE_POSSESSION_VERTICAL_SLICE_SPEC.md
docs/2-execution/19_ENGINEERING_ROADMAP_TUI_FIRST.md
docs/2-execution/20_TESTING_VALIDATION_AND_DEBUGGING.md
docs/2-execution/21_AUTHORING_GUIDE.md
docs/2-execution/22_STARTER_DATA_SCHEMAS.md
```

External source pointers used by the authority docs and execution notes:

```text
Event sourcing and CQRS:
- https://martinfowler.com/eaaDev/EventSourcing.html
- https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing
- https://learn.microsoft.com/en-us/azure/architecture/patterns/cqrs

Deterministic simulation/replay:
- https://gafferongames.com/post/deterministic_lockstep/
- https://gafferongames.com/post/floating_point_determinism/
- https://randomascii.wordpress.com/2013/07/16/floating-point-determinism/

BDI/HTN/GOAP:
- https://cdn.aaai.org/ICMAS/1995/ICMAS95-042.pdf
- https://link.springer.com/chapter/10.1007/3-540-49057-4_1
- https://dl.acm.org/doi/book/10.5555/975615
- https://www.gamedevs.org/uploads/three-states-plan-ai-of-fear.pdf

Social simulation/emergent narrative:
- https://www.gameaipro.com/GameAIPro3/GameAIPro3_Chapter37_Simulating_Character_Knowledge_Phenomena_in_Talk_of_the_Town.pdf
- https://www.ben-samuel.com/wp-content/uploads/2015/09/FDG-2011-Prom-Week-Social-Physics-as-Gameplay.pdf
- https://ieee-cog.org/2022/assets/papers/paper_122.pdf
- https://github.com/ShiJbey/neighborly

Normative MAS and LOD:
- https://icr.uni.lu/leonvandertorre/papers/aisb05.pdf
- https://pmc.ncbi.nlm.nih.gov/articles/PMC4119705/
- https://www.ifaamas.org/Proceedings/aamas2011/papers/C5_B67.pdf
- https://hal.science/hal-01691388/document

LLM social simulation:
- https://arxiv.org/abs/2304.03442
- https://arxiv.org/abs/2507.19364
- https://arxiv.org/abs/2506.19806
- https://arxiv.org/abs/2507.06310
- https://arxiv.org/abs/2510.21180

Game/simulation precedents and counterexamples:
- https://www.bay12games.com/dwarves/dev.html
- https://dwarffortresswiki.org/index.php/World_generation
- https://colepowered.com/shadows-of-doubt-devblog-8-simulating-a-city/
- https://store.steampowered.com/app/986130/Shadows_of_Doubt/
- https://steamcdn-a.akamaihd.net/apps/valve/2009/ai_systems_of_l4d_mike_booth.pdf
- https://rimworldwiki.com/wiki/AI_Storytellers
- https://www.wired.com/2011/11/skyrim-infinite-quests/

Rust/TUI/testing ecosystem candidates:
- https://www.rust-lang.org/
- https://ratatui.rs/
- https://github.com/crossterm-rs/crossterm
- https://github.com/proptest-rs/proptest
- https://insta.rs/
- https://nexte.st/
- https://bheisler.github.io/criterion.rs/book/
- https://serde.rs/
```

## Final execution conclusion

The replacement execution set is intentionally smaller and harsher than the old execution set.

The first deliverable is not a dramatic village. It is a village that can be audited.

The correct order is:

```text
paper ontology
 -> physical/event/TUI spine
 -> epistemics/view/possession parity
 -> ordinary needs/routines/no-human day
 -> local authority/report/record/wrong suspicion
 -> second proof notices/travel/regional expansion
```

Any implementation path that reaches road-threat adventure before the village can explain missing property, wrong belief, fallible record, no-human routine, and possession parity is taking the wrong path.
