# Spec 0002 — Phase 1 Kernel, TUI, Event Log, and Replay Implementation Specification

**Status:** Implementable Phase 1 specification  
**Repository:** `joeloverbeck/tracewake`  
**Target commit:** `841deeb6fc73f8006ed2548530c062067d4f5250`  
**Intended repository path:** `docs/4-specs/0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md`  
**Deliverable filename:** `0002_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY_IMPLEMENTATION_SPEC.md`  
**Phase:** Phase 1, following Spec 0001 / Phase 0  
**Normative posture:** Narrow vertical slice; reusable seams only where they directly protect Phase 1 determinism, replay, TUI playability, validation, and future epistemic correctness.

I am not verifying that this commit is the current `main`. I am using your supplied commit as the target of record and fetching files only by exact commit URL from `joeloverbeck/tracewake`.

---

## 1. Evidence ledger

Requested repository: joeloverbeck/tracewake  
Target commit: 841deeb6fc73f8006ed2548530c062067d4f5250  
Freshness claim: user-supplied target commit only; not independently verified as latest main  
Manifest role: path inventory only  
Repository metadata used: no  
Default-branch lookup used: no  
Branch-name file fetch used: no  
Code search used: no  
Clone used: no  
URL fetch method: `web.run.open` against full exact raw URLs; `container.download` only against exact raw URLs after successful exact-URL viewing for local artifact preparation; no repository-scoped fetch arguments  
Fetched files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/1-architecture/01_SYSTEM_AUTHORITY_RUST_WORKSPACE_AND_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/1-architecture/02_ACTION_AFFORDANCE_SCHEDULING_AND_PROPOSAL_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/1-architecture/03_EVENT_LOG_REPLAY_PROJECTIONS_AND_SAVE_PACKAGES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/1-architecture/04_STATE_CONTENT_AUTHORING_DOMAIN_PACKS_AND_SCHEMA_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/1-architecture/05_CLAIMS_BELIEFS_TRACES_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/1-architecture/06_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/1-architecture/07_AGENT_COGNITION_ROUTINES_PLANNING_AND_AGENT_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/1-architecture/09_ORDINARY_LIFE_ECONOMY_SETTLEMENT_AND_SPATIAL_MODEL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/1-architecture/11_INCIDENTS_LEADS_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_WORLD_HISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/1-architecture/13_VALIDATION_OBSERVABILITY_METRICS_AND_ACCEPTANCE_ARCHITECTURE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/1-architecture/14_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/2-execution/01_EXECUTION_CHARTER_SCOPE_AND_FIRST_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/2-execution/02_PHASE_LADDER_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/2-execution/03_FIRST_PROOF_MISSING_EXPECTED_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/2-execution/04_PHASE_0_PAPER_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/2-execution/05_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/2-execution/06_PHASE_2_EPISTEMICS_VIEW_MODELS_AND_POSSESSION_PARITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/2-execution/07_PHASE_3_NEEDS_ROUTINES_AND_NO_HUMAN_LIFE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/2-execution/08_PHASE_4_INSTITUTIONS_RECORDS_AND_WRONG_SUSPICION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/2-execution/09_DATA_AUTHORING_VALIDATION_AND_GOLDEN_FIXTURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/2-execution/10_TESTING_DEBUGGING_OBSERVABILITY_AND_METRICS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/2-execution/11_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_AND_REGIONAL_EXPANSION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/2-execution/12_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/4-specs/0001_FOUNDATIONAL_DOC_AMENDMENTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/4-specs/0001_PHASE_0_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/4-specs/0001_RESEARCH_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/docs/4-specs/SPEC_LEDGER.md
Contamination observed: no  
Connector/tool namespace trusted as evidence: no

The uploaded manifest was used only to enumerate paths. It was not used as source text for product or architecture authority. Every source document considered by this specification was fetched mechanically as:

`https://raw.githubusercontent.com/joeloverbeck/tracewake/841deeb6fc73f8006ed2548530c062067d4f5250/<manifest path>`

---

## 2. Source authority summary

This specification follows the repository's documented authority ladder:

1. **Foundation documents** bind the product identity and non-negotiable invariants.
2. **Architecture documents** bind system boundaries, data/event/replay contracts, workspace direction, validation, TUI, possession, and debug separation.
3. **Execution documents** bind phase order and Phase 1 acceptance gates.
4. **Reference documents** bind terminology, exact-source discipline, review checklists, and known design risks.
5. **Spec 0001** binds the Phase 0 ontology, first-proof fixtures, primitive vocabulary, view-model sketches, no-scripting review, and Phase 1 entry contract.

If this document conflicts with foundation text, this document is wrong. If it conflicts with architecture, this document is wrong unless the conflict is a deliberate, narrower Phase 1 choice that does not weaken the architecture. If it conflicts with execution, this document must be read as the implementable Phase 1 narrowing of execution, not as permission to add Phase 2+ systems.

The most important binding source commitments are:

| Source layer | Binding consequence for Phase 1 |
|---|---|
| Foundation | Tracewake is a causality-first life simulation, not a quest engine, protagonist mystery, story director, LLM chatbot, or TUI-less backend. The world must not know which actor, if any, is controlled by a human. Meaningful changes require events. Replay and debug explanation are foundational, not optional polish. |
| Architecture | Rust is authoritative. The kernel owns rules. UI, debug, replay, and content tooling must not mutate state behind the kernel. All actions enter one proposal/validation/commit path. Event envelopes are versioned from the first event. Projections are derived and rebuildable. Content is causal possibility, not outcome script. |
| Execution | Phase 1 is the smallest runnable physical/action/event/TUI/replay spine: move, inspect, open/close, take/place, wait, see why-not, debug item location, rebuild projection, replay deterministically, and run no-human. |
| Reference | Exact-source discipline and terminology are part of the product guardrail. `PlayerCharacter`, `Quest`, `Objective`, `Reward`, hidden story scripting, and debug truth leakage are design risks, not harmless shortcuts. |
| Spec 0001 | Phase 0 supplies the ontology and fixture contract: actors, places, rooms, doors, containers, physical value tokens, ownership/custody/access distinctions, primitive actions/events, view-model separation, no-scripting checks, and Phase 1 entry requirements. |

This specification also incorporates the resolved implementation intent for Phase 1: build the smallest serious runnable vertical slice with reusable seams; use a compact Rust workspace; make the TUI genuinely playable through stable semantic actions; make replay determinism mandatory; serialize event-log and test fixtures now; validate fixture/content data now; include the seven required Phase 1 golden fixtures; provide structured debug reports for item location and action rejection; enforce possession parity; and keep Phase 1's fun focused on forensic playability rather than social or narrative completeness.

---

## 3. Phase 1 purpose

Phase 1 proves that Tracewake has a real causal kernel, not a paper ontology and not a UI demo.

A passing Phase 1 lets a human bind a controller to an ordinary actor, view a local place through that actor, choose stable semantic actions, move locally, inspect, open and close doors or containers, take and place physical items, wait, see structured reasons when actions are impossible, inspect debug provenance, rebuild projections from an initial fixture plus ordered events, and replay to the same physical result.

The proof is intentionally small. It is not the missing-property epistemic proof yet. It is the physical, eventful, replayable substrate that makes the later missing-property proof honest.

Phase 1 is successful only if all of the following are true at once:

- a non-programmer can play the fixture through a crude TUI using ordinary semantic commands;
- an automated test can run the same actions without the TUI and receive the same validation and event results;
- a no-human run can advance without a sacred player entity or player-only branch;
- every meaningful physical change is represented by a versioned event envelope;
- a projection can be rebuilt from the fixture plus ordered events;
- replay produces a matching physical state and deterministic checksum/report;
- debug can answer “where is this item and what event put it there?” and “why was this action rejected?” without leaking that truth into embodied actor state;
- content validation rejects player-shaped, quest-shaped, script-shaped, and nondeterministic data before it can become accepted fixture content.

The Phase 1 experience should feel like forensic playability: ordinary local movement, physical manipulation, visible failure, event traces, and replayable cause. It must not pretend to offer full social life, suspicion, belief contradiction, institutional procedure, or narrative drama.

---

## 4. Phase 1 scope

Phase 1 implements exactly the narrow physical/action/event/TUI/replay slice described below.

### 4.1 Required implementation areas

Phase 1 must implement:

1. **A compact Rust workspace** with a small authoritative kernel and clear module boundaries.
2. **A deterministic simulation core** with discrete ticks, stable ordering, stable IDs, deterministic fixture loading, and no outcome dependence on wall-clock time, thread races, process randomness, terminal timing, hash iteration order, network data, or LLM output.
3. **Minimal entity/component/state support** for actors, places, doors, containers, physical items/value tokens, locations, ownership/custody placeholders, and controller-binding metadata outside world state.
4. **Fixture/content validation** for Phase 1 data, including IDs, references, target kinds, location consistency, unsupported action targets, nondeterministic ordering hazards, and forbidden player/quest/script content.
5. **One shared command/proposal/action validation pipeline** used by human commands, non-human proposals, tests, and no-human scheduler processes.
6. **A TUI shell** that consumes view models and submits stable semantic commands. The TUI may be ugly. It may not be fake. It must not mutate world state or contain simulation rules.
7. **Local movement** between fixture places through validated adjacency and door constraints.
8. **Inspect actions** for the current place and visible or locally reachable objects.
9. **Open/close actions** for doors and containers where physically valid.
10. **Take/place actions** for physical items where physically valid.
11. **Wait/advance time** through the same causal scheduler/time machinery used without a human.
12. **Structured action rejection reports** with stable reason codes and relevant provenance.
13. **Append-only event logging** for meaningful physical state changes and versioned non-world diagnostic/control events where required for replay/debug/test observability.
14. **Versioned event envelopes** from the first event.
15. **Projection rebuild** from the initial fixture plus ordered events.
16. **Replay checksums or equivalent replay reports** that prove replayed physical state matches the original run.
17. **Debug attachment and inspection** for controller binding, item location, event log, projection rebuild, and action rejection provenance.
18. **No-human advance** that runs without a controller and without sacred player state.
19. **Golden fixtures and automated tests** proving the entire slice.

### 4.2 Required Phase 1 golden fixtures

The executable Phase 1 fixture suite must include all seven of these fixture IDs:

- `strongbox_001`
- `container_item_move_001`
- `door_access_001`
- `debug_attach_001`
- `no_human_advance_001`
- `replay_item_location_001`
- `view_model_local_actions_001`

The fixtures may share underlying content where useful, but each fixture must have its own explicit scenario contract, expected actions, expected events or reports, and acceptance assertions.

### 4.3 Phase 1 implementation boundary

Phase 1 may create future-facing placeholders only when needed to preserve Phase 0 contracts. Placeholders must be inert, validated, and visibly incomplete. A placeholder must never masquerade as an implemented Phase 2+ system.

Allowed placeholders include:

- ownership/custody fields needed to separate legal owner, current holder, permitted access, and expected-location stub;
- institution or record entity IDs if needed by Phase 0 fixture shape, but with no institutional logic;
- proposition or claim IDs if needed to preserve Spec 0001 data shape, but with no belief-update engine;
- debug-visible event/projection metadata needed by later epistemic projections.

Any placeholder that changes validation, action availability, replay, or actor view output must be explicitly tested. A placeholder that silently changes behavior is not a placeholder; it is an undeclared feature.

---

## 5. Phase 1 non-goals

Phase 1 must not implement, simulate, fake, or partially smuggle in any of the following:

- full belief stores;
- expectation contradiction;
- actor notebooks, except inert future-facing placeholders required by content shape;
- observation interpretation;
- hidden epistemic filtering beyond minimal embodied/debug separation;
- hunger;
- fatigue;
- safety;
- sleep;
- jobs;
- needs;
- routines;
- planner cognition;
- autonomous social life;
- institutions;
- reports;
- ledgers as active systems;
- suspicion;
- questioning;
- sanctions;
- wrong-suspicion proof;
- rumors;
- testimony;
- records as institutional truth-making systems;
- notices;
- stale leads;
- travel beyond local adjacent movement;
- regional simulation;
- LLM speech;
- LLM parsing;
- LLM-rendered prose;
- LLM authority over state, action validation, observations, events, or content;
- combat;
- magic;
- quests;
- protagonist privilege;
- drama director logic;
- authored outcome chains;
- graphical client;
- durable save-game UX beyond replay/test artifacts;
- broad ECS/framework architecture unless it is demonstrably minimal and phase-safe.

Phase 1 also must not define a `PlayerCharacter`, a player inventory, player-exclusive verbs, player objectives, reward loops, quest markers, culprit pointers, story beats, or content rules that fire because the human arrived, inspected, waited, or became bored.

---

## 6. Binding invariants from the foundation docs

The following invariants are binding for Phase 1. They are not suggestions.

### 6.1 The world ignores the human

The authoritative world state must not contain a sacred player entity. A human controller may be bound to an ordinary actor, but the world must not know or care whether that actor is controlled by a human.

Controller binding is runtime/control metadata. It is not a physical fact, social fact, mental fact, or epistemic fact in the simulation. Possession must not transfer knowledge. Possession must not mark the actor as special. Possession must not unlock verbs that non-human actors cannot use.

### 6.2 Every player world action must be NPC-possible

If the human can move, open, close, take, place, inspect, or wait through the TUI, the same action definition must be available to a non-human proposal under the same physical preconditions. The only human-specific operations are non-world controller/debug operations, such as attaching the controller or opening a debug panel.

### 6.3 Meaningful state changes require events

Phase 1 may not mutate authoritative physical state directly as a side effect of a command, UI callback, fixture script, debug command, test helper, or projection rebuild. Accepted physical changes must be represented as ordered, versioned event envelopes and applied through the event application path.

### 6.4 Replay is foundational

Replay determinism is not a later optimization. Phase 1 fails if it cannot rebuild physical state and projections from the initial fixture plus ordered events and produce a matching checksum/report.

### 6.5 TUI-first means runnable phases are playable

A passing Phase 1 must expose the slice through the TUI. A test-only or debug-command-only kernel is not sufficient. The TUI does not need beauty, prose, or convenience. It does need stable semantic actions, local view models, why-not, wait/advance, debug access, and transcript/snapshot coverage.

### 6.6 Why-not is mandatory

Invalid actions must produce structured rejections. A failed action that returns only “no,” silently ignores input, or prints an unstructured string does not satisfy Phase 1.

### 6.7 Debug is non-diegetic

Debug truth must be visibly non-diegetic and must not enter embodied actor state. Debug reports may show actual item location, event ancestry, validation internals, and projection checksums. Actor views may not inherit those facts merely because debug was opened or the controller was rebound.

### 6.8 LLMs have no Phase 1 authority

No Phase 1 behavior may require an LLM. No LLM may validate actions, interpret commands, author events, mutate state, generate canonical descriptions, fill data, decide outcomes, or repair content. LLM-disabled acceptance tests must pass.

### 6.9 No authored outcome chain

Content may define initial causal possibilities. It may not define ordinary playable outcomes such as “when Tomas opens the strongbox, make the coins missing,” “after three waits, move the coins,” or “if the human enters the room, trigger suspicion.” Such content is scripting and must be rejected.

---

## 7. Binding architecture constraints

### 7.1 Kernel authority

The Rust kernel owns authoritative world rules. The TUI, fixture loader, debug tools, replay runner, projections, and tests are clients of kernel APIs. They must not contain duplicate action rules or hidden state mutation paths.

Required boundary:

- command intake creates an action proposal or a non-world debug/control command;
- action proposal validation happens in the shared action pipeline;
- accepted world proposals produce event envelopes;
- event application mutates authoritative state;
- projections derive views from authoritative state and event history;
- TUI renders view models and submits semantic commands;
- debug reads state, events, validation reports, and replay reports without mutating world facts.

### 7.2 Dependency direction

The workspace must enforce dependency direction. The kernel may expose domain types, state, validation, events, replay, projections, and debug report builders. The TUI may depend on the kernel and content loader. The kernel must not depend on terminal libraries, command-line UI libraries, fixture file paths, wall-clock time, network services, or LLM clients.

### 7.3 Deterministic side-effect policy

Outcome-affecting code may not use:

- wall-clock time;
- OS/process randomness;
- unrecorded random draws;
- network data;
- live LLM calls;
- filesystem reads during action resolution;
- thread scheduling races;
- Rust `HashMap` or `HashSet` iteration order for outcome-affecting iteration;
- terminal frame timing;
- display order as an implicit semantic target;
- debug UI state as simulation input.

### 7.4 Event-sourced mutation

State mutation must be routed through event application. The implementation may maintain an in-memory current state for playability, but that state must be reproducible from fixture plus ordered events. Projection rebuild must not depend on a saved current-state snapshot.

### 7.5 Content is possibility, not script

Content defines entities, relationships, initial state, permitted physical affordances, and fixture setup. Content must not encode authored outcomes. Domain packs and fixtures must be validated before play or tests.

### 7.6 TUI/view-model separation

The TUI must render view models produced by the kernel/projection layer. The TUI may format, paginate, highlight, and dispatch selected semantic action IDs. It may not infer rules, discover hidden entities, mutate state, or create action availability independent of the action registry/view model.

### 7.7 Debug separation

Debug surfaces may expose truth. They must be marked as debug, routed through debug APIs, and tested for non-leakage into embodied state. Debug commands that affect controller binding or replay/projection tools are not ordinary world actions.

---

## 8. Relationship to Spec 0001 / Phase 0 contract

Spec 0001 is the Phase 0 reference contract. Phase 1 implements a narrow executable subset of that contract.

### 8.1 Phase 0 inputs consumed by Phase 1

Phase 1 must consume these Phase 0 outputs:

- primitive action vocabulary, with debug possession switching separated from ordinary play;
- primitive event vocabulary for physical state, replay, and future projections;
- first village minimal fixture contract with actors, places, rooms, doors, containers, and `coin_stack_01`;
- stable ID policy: content-stable semantic identifiers, not display names and not final storage keys;
- embodied and debug TUI view-model boundaries;
- no-scripting review checklist;
- validation requirements for actors, places, doors, containers, and items;
- fixture contracts, especially `strongbox_001` and replay rebuild expectations.

### 8.2 What Phase 1 implements from Spec 0001

Phase 1 implements only the physically executable parts:

- actors as bodies capable of ordinary actions;
- places/rooms as local movement graph nodes;
- doors as physical blockers/openable objects;
- containers as physical holders/openable objects;
- items/value tokens as physical entities with one current location;
- ownership/custody/access fields as data and validation facts, not as full social/legal systems;
- action proposals and validation for local physical actions;
- event envelopes for physical changes, rejections where required, and replay/debug control events;
- embodied/debug view models at the minimum needed for play and test;
- no-scripting validation.

### 8.3 What Phase 1 must not fake from Spec 0001

Phase 1 must not implement fake versions of later proof systems. In particular:

- `strongbox_001` may prove that `coin_stack_01` begins in `strongbox_tomas`, can be moved through ordinary events, can be located through debug, and can be replayed. It must not prove Tomas's missing-property belief contradiction.
- Ownership/custody fields may be validated and displayed/debugged. They must not become institutional enforcement, suspicion, sanctions, or report acceptance logic.
- Actor views may show local physical visibility. They must not pretend to be full subjective belief views.
- Records, reports, notebooks, claims, and institutions may exist only as inert placeholders if the fixture shape requires them. They must not do work.

### 8.4 Spec 0001 compatibility tests

Phase 1 must include compatibility tests proving:

- `strongbox_001` loads with the required physical IDs and relationships;
- `coin_stack_01` is a physical item/value token, not an abstract balance;
- ownership, possession/current holder, custody, and expected-location stub are distinct fields;
- debug possession switching is separated from ordinary play actions;
- forbidden content terms and outcome scripts are rejected;
- Phase 1 does not silently collapse Phase 0's future epistemic distinctions into physical truth.

---

## 9. Recommended Rust workspace shape and module boundaries

Use a compact workspace. Do not over-split crates in Phase 1. Tracewake needs real seams, not architecture cosplay.

### 9.1 Required workspace recommendation

Recommended shape:

| Workspace member | Kind | Responsibility |
|---|---|---|
| `crates/tracewake-core` | library | Authoritative domain types, IDs, state, deterministic scheduler/time primitives, action registry, proposal validation, mutation plans, event envelopes, event application, replay, projections, checksums, debug report builders. |
| `crates/tracewake-content` | library | Fixture/domain-pack schemas, deterministic loading, content validation, canonical content manifests, golden fixture constructors/readers, test fixture serialization. Depends on `tracewake-core`; never on TUI. |
| `crates/tracewake-tui` | binary/library as needed | Terminal shell, command input, view-model rendering, semantic action selection, debug panels, transcript/snapshot harness. Depends on `tracewake-core` and `tracewake-content`; never owns rules. |

Optional for developer ergonomics, but not required for Phase 1 exit:

| Optional member | Allowed use |
|---|---|
| `crates/tracewake-cli` | Thin commands such as `validate-fixture`, `replay-log`, `checksum`, or `run-golden`. It must call the same core/content APIs used by tests. It must not become a second implementation. |

Do not split `events`, `replay`, `projections`, `debug`, or `scheduler` into separate crates during Phase 1 unless there is an immediate compile-time boundary or test isolation need. They should start as modules inside `tracewake-core`.

### 9.2 Required core modules

`tracewake-core` should expose modules with these responsibilities:

| Module | Phase 1 responsibility |
|---|---|
| `ids` | Stable ID newtypes or equivalent for actors, places, doors, containers, items, actions, events, validation reports, fixtures, content versions, controller bindings. IDs must be comparable and serializable canonically. |
| `state` | Authoritative physical state records and component storage. No terminal/debug concerns. |
| `location` | Single-source location model for items and contained objects. Prevent double-location by construction and validation. |
| `time` | Discrete simulation ticks and deterministic ordering keys. No wall-clock authority. |
| `actions` | Action definitions, semantic action IDs, proposal types, registry, target binding, validation pipeline, mutation plan construction. |
| `events` | Versioned event envelopes, event kind registry, append-only log, event application, event schema/version handling. |
| `scheduler` | Minimal deterministic scheduling and no-human advance. Phase 1 scheduler may be simple, but must use stable ordering. |
| `projections` | Derived physical/local/debug projection builders; projection rebuild from initial fixture plus event stream. |
| `view_models` | Embodied and debug view-model data structures consumed by TUI and tests. No terminal rendering. |
| `debug_reports` | Structured item-location, action-rejection, projection-rebuild, replay, and controller-binding reports. |
| `checksum` | Canonical deterministic physical-state and projection checksum/report generation. |
| `validation` | Runtime invariant checks shared by tests, content validation, replay, and debug diagnostics. |

### 9.3 Required content modules

`tracewake-content` should expose:

| Module | Phase 1 responsibility |
|---|---|
| `schema` | Serialized fixture/domain-pack data structures. These may mirror core types but should make invalid authoring detectable. |
| `load` | Deterministic file/package loading and canonical ordering. |
| `validate` | Full content validation before producing an accepted `InitialWorld` or equivalent. |
| `manifest` | Content package identity, fixture ID, schema version, content version, canonical path order, checksum/fingerprint. |
| `fixtures` | Seven golden fixture definitions or fixture readers with explicit scenario contracts. |
| `serialization` | Test fixture and event-log serialization format; stable enough for regression fixtures. |

### 9.4 Required TUI modules

`tracewake-tui` should expose or contain:

| Module | Phase 1 responsibility |
|---|---|
| `app` | Runtime loop around view model rendering and semantic command submission. |
| `render` | Terminal display only. May be ugly. Must not own rules. |
| `input` | Map typed keys/commands/list selections to stable semantic action IDs or debug/control commands. |
| `debug_panels` | Debug event log, item-location report, rejection report, replay/projection rebuild, controller binding status. |
| `transcript` | Deterministic transcript/snapshot harness for tests. |

### 9.5 Dependency rules

The following dependency rules are mandatory:

- `tracewake-core` must not depend on `tracewake-content`, `tracewake-tui`, terminal libraries, network libraries, live LLM clients, or wall-clock APIs for outcome logic.
- `tracewake-content` may depend on `tracewake-core` for IDs/domain types and validation outputs.
- `tracewake-tui` may depend on `tracewake-core` and `tracewake-content`.
- Tests may use all crates, but golden assertions must exercise public APIs that a real client can use.
- No crate may provide a second action validator or event applier.

---

## 10. Determinism contract

Phase 1 determinism is an acceptance gate. It is not best effort.

### 10.1 Stable identifiers

Every persistent fixture entity must have a stable semantic ID. Phase 1 requires stable IDs for:

- fixtures;
- actors;
- places/rooms;
- exits/adjacency edges where represented directly;
- doors;
- containers;
- items/value tokens;
- action definitions;
- semantic action instances in view models;
- event envelopes;
- validation reports;
- controller bindings as runtime metadata;
- content manifests and schema versions.

Display names are not IDs. Array positions are not IDs. Terminal menu indices are not IDs. Generated UUIDs are not acceptable for deterministic world entities unless generated from stable content path, parent ID, and stable ordinal using a documented deterministic derivation.

### 10.2 Deterministic loading order

The fixture loader must produce the same accepted initial world across runs and platforms.

Required rules:

- input paths from a content package must be ordered canonically before loading;
- entities must be normalized into deterministic order, preferably lexicographic by stable ID;
- any author-specified order that affects behavior must be explicit and validated;
- duplicate IDs are fatal validation errors;
- references must be resolved after canonicalization, not by incidental parse order;
- if serialized maps are used, loader logic must not depend on serialized map iteration order;
- validation errors must be sorted deterministically before reporting.

### 10.3 Deterministic action ordering

When multiple proposals or scheduled actions are present, outcome order must be determined by an explicit ordering key:

1. simulation tick;
2. scheduled phase or source class, if needed and documented;
3. actor/process stable ID;
4. proposal sequence number assigned deterministically at intake;
5. action definition ID;
6. target ID tuple;
7. final deterministic tie-breaker.

No outcome may depend on insertion order into an unordered collection, OS scheduling, terminal timing, or arbitrary test execution order.

### 10.4 Time

Phase 1 uses discrete simulation ticks. Wall-clock time is not simulation authority.

`wait` and no-human advance must use the same tick model. If a wait advances simulation time, that advancement is meaningful for causal order and must be represented by an event or an event-stream position that replay can verify.

### 10.5 Randomness

Phase 1 should avoid randomness unless a fixture explicitly requires a deterministic draw. If randomness is used:

- random streams must be seeded by fixture/run metadata;
- draws must be scoped, named, and ordered deterministically;
- meaningful draws must be recorded in event envelopes or replay diagnostics;
- replay must fail loudly if a random draw differs.

Hidden global RNG is forbidden.

### 10.6 Canonical checksums

Phase 1 must implement a deterministic physical-state checksum or equivalent replay report. The checksum input must be canonical and must exclude non-world runtime metadata such as current terminal layout, active debug panel, and current human controller binding unless the report explicitly computes a separate run-metadata checksum.

The physical checksum must include, at minimum:

- fixture ID and content version/fingerprint;
- simulation tick/order position;
- entity IDs and kinds;
- actor locations and carried items;
- place graph state relevant to Phase 1;
- door open/closed/locked state;
- container open/closed/locked state and ordered contents;
- item current holder/location;
- ownership/custody/access placeholder fields if they affect validation or debug;
- event stream position applied.

### 10.7 Determinism test requirement

Every golden fixture must have a deterministic load test. Every eventful golden scenario must have a replay determinism test. At least one test must run the same scenario twice in one process and once through serialization/reload, then compare checksums/reports.

---

## 11. Event log and replay contract

### 11.1 Event streams

Phase 1 must distinguish world events from non-world run/debug/control events.

| Stream | Purpose | May mutate physical world? | Examples |
|---|---:|---:|---|
| `world` | Authoritative meaningful simulation changes and causal time advancement. | yes, by event application only | actor moved, door opened, item moved, time advanced |
| `diagnostic` | Structured validation/debug facts needed for why-not and test observability. | no | action rejected, validation report stored |
| `controller` | Runtime controller binding metadata. | no | controller attached/switch/detach |
| `replay_debug` | Replay/projection rebuild diagnostics. | no | projection rebuilt, replay report generated |

All streams use versioned event envelopes. Only the `world` stream contributes to physical world reconstruction. Non-world streams may be serialized for tests and developer UX, but they must not become embodied facts.

### 11.2 Event envelope version 1

Every event envelope in Phase 1 must carry these fields or their explicit equivalents:

| Field | Requirement |
|---|---|
| `event_id` | Stable deterministic event ID or stream position-derived ID. Must be unique within the event package. |
| `event_type` | Stable event kind ID. No display strings as type authority. |
| `event_schema_version` | Present from the first event. Unknown versions fail replay unless an explicit migrator exists. |
| `stream` | `world`, `diagnostic`, `controller`, or `replay_debug`. |
| `stream_position` | Monotonic ordered position within stream. |
| `global_order` | Deterministic cross-stream order if streams are serialized together. |
| `sim_tick` | Discrete simulation tick relevant to the event. |
| `ordering_key` | Deterministic scheduler/proposal ordering data sufficient to explain order. |
| `actor_id` | Present for actor-originated world actions; absent or null for process/debug events. |
| `process_id` | Present for no-human or replay/debug process events where no actor is responsible. |
| `participants` | Stable IDs of entities materially involved. Sorted or semantically ordered. |
| `place_id` | Local place context where relevant. |
| `causes` | Prior event IDs, proposal IDs, validation report IDs, or process causes. Empty only when explicitly root/fixture-derived. |
| `proposal_id` | For events produced by action proposals. |
| `validation_report_id` | For accepted or rejected actions where validation report is retained. |
| `random_draws` | Empty unless deterministic randomness was used; otherwise exact draw references. |
| `payload` | Event-kind-specific state delta data. Must be canonical serializable data, not prose authority. |
| `effects_summary` | Debug/test summary of physical effects. Not authoritative over payload. |
| `content_manifest_id` | Fixture/content package identity used for validation/replay compatibility. |
| `checksum_after` | Required for world events by the end of Phase 1, or replaced by a replay report that records equivalent after-event fingerprints. |

### 11.3 Required Phase 1 event kinds

Phase 1 must support these event kinds or exact documented aliases with the same semantics:

| Event kind | Stream | Semantics |
|---|---|---|
| `ControllerAttached` | `controller` | Human/debug controller binding changed. This is not a world fact. It must not alter actor knowledge, physical state, or validation rules. |
| `ControllerDetached` | `controller` | Optional but recommended. Binding removed. Not a world fact. |
| `ActorMoved` | `world` | Actor changed place through validated adjacency and blockers. |
| `DoorOpened` | `world` | Door changed from closed to open. |
| `DoorClosed` | `world` | Door changed from open to closed. |
| `ContainerOpened` | `world` | Container changed from closed to open. |
| `ContainerClosed` | `world` | Container changed from open to closed. |
| `ItemRemovedFromContainer` | `world` | Item moved from a container to an actor/carrier through a validated take action. |
| `ItemTakenFromPlace` | `world` | Item moved from a local place to an actor/carrier through a validated take action. |
| `ItemPlacedInContainer` | `world` | Item moved from an actor/carrier into a container through a validated place action. |
| `ItemPlacedInPlace` | `world` | Item moved from an actor/carrier into the local place through a validated place action. |
| `ActorWaited` or `TimeAdvanced` | `world` | A wait/advance changed simulation tick/order. If the implementation separates actor wait from global time advancement, both must replay deterministically. |
| `ActionStarted` | `world` or `diagnostic` | Required only for scheduled/durational actions introduced in Phase 1. If all actions are immediate, this may be inert but versioned. |
| `ActionFailed` | `world` or `diagnostic` | Runtime failure after start, distinct from validation rejection. Required only if scheduled/durational actions can fail after acceptance. |
| `ActionRejected` | `diagnostic` | Structured rejected proposal report. Must not mutate world state. Required for golden why-not/debug tests. |
| `NoHumanAdvanceStarted` | `diagnostic` or `world` process marker | Marks no-human advance run. Must not imply player absence as a world fact. |
| `NoHumanAdvanceCompleted` | `diagnostic` or `world` process marker | Recommended for replay/debug reports. |
| `ReplayProjectionRebuilt` | `replay_debug` | Projection rebuild report was generated. Must not mutate world state. |

Inspect/look actions do not create world events in Phase 1 unless they later become observation events in Phase 2. For Phase 1, inspect is a view/query action with validation and a view-model response. It must not create actor knowledge.

### 11.4 Append-only log

The event log is append-only. Phase 1 may store it in memory during a TUI run, but it must also be serializable for replay fixtures and regression tests.

The implementation may not:

- rewrite past event payloads;
- reorder events after commit;
- silently drop unknown event versions;
- apply events without validating pre-event state compatibility;
- skip event creation for meaningful accepted world changes;
- repair divergent replay by mutating current state directly.

### 11.5 Event application

World event application must be deterministic and strict.

Required behavior:

- verify the event schema version is supported;
- verify referenced entities exist and have required kinds/components;
- verify old state in event payload, or equivalent precondition summary, matches current replay state where applicable;
- apply only the event's declared state changes;
- update derived indexes deterministically;
- update checksum/report input;
- fail loudly on impossible event application.

An event applier that “best effort” applies inconsistent events is a corruption engine. It is forbidden.

### 11.6 Projection rebuild

A projection rebuild must start from an accepted initial fixture and ordered event stream. It must not depend on the live mutable projection. It must produce:

- final physical state or projection;
- event count applied;
- last event ID/position;
- content manifest identity;
- final checksum/fingerprint;
- list of unsupported event versions, if any;
- invariant violations, if any;
- diff against expected live/final state where used in tests.

Projection rebuild must be available through automated tests and debug UI.

### 11.7 Replay report

Replay must produce a structured report with, at minimum:

| Field | Meaning |
|---|---|
| `fixture_id` | Initial fixture used. |
| `content_manifest_id` | Content version/fingerprint. |
| `initial_checksum` | Canonical initial physical checksum. |
| `event_count` | Number of world events applied. |
| `diagnostic_event_count` | Number of diagnostic/controller/replay events loaded, if included. |
| `unsupported_versions` | Empty for passing Phase 1. |
| `application_errors` | Empty for passing Phase 1. |
| `final_checksum` | Canonical final physical checksum. |
| `expected_checksum` | Optional expected checksum from fixture/test artifact. |
| `matches_expected` | Boolean. Must be true for passing replay tests. |
| `state_diff` | Empty for matching replay. Deterministically ordered if non-empty. |

### 11.8 Replay acceptance

Phase 1 fails if any golden scenario can only pass by comparing mutable current state and cannot be rebuilt from event log. Phase 1 also fails if replay accepts checksum drift as a warning.

---

## 12. Entity/component/state model for Phase 1 only

Phase 1 must use an explicit, minimal state model. Do not introduce a broad ECS unless it is clearly smaller and safer than typed records for this slice.

### 12.1 Entity kinds

Required entity kinds:

| Kind | Purpose |
|---|---|
| `Actor` | Ordinary world participant capable of Phase 1 actions. Human possession may bind to an actor but does not change the actor. |
| `Place` | Local graph node: room, workplace, public room, office placeholder, or equivalent local area. |
| `Door` | Physical connector/blocker between places. May be open/closed and optionally locked. |
| `Container` | Physical holder for items. May be open/closed and optionally locked. |
| `Item` | Physical movable item. Includes value-token items such as `coin_stack_01`; no abstract balances. |
| `InstitutionPlaceholder` | Optional inert placeholder if required by fixture shape. No active institutional logic. |
| `RecordPlaceholder` | Optional inert placeholder if required by fixture shape. No report/ledger truth logic. |

Forbidden Phase 1 entity kinds include `PlayerCharacter`, `Quest`, `Objective`, `Reward`, `Culprit`, `StoryBeat`, `ScriptTrigger`, and any equivalent disguised product term.

### 12.2 Core components/records

Required components or equivalent typed records:

| Component/record | Required fields/concepts |
|---|---|
| `EntityHeader` | stable ID, entity kind, stable display label, optional tags restricted by validation. |
| `ActorBody` | actor ID, current place ID, carried item IDs in deterministic order, carry capacity or equivalent physical limit, alive/enabled flag if needed. |
| `PlaceState` | place ID, display label, local entity memberships or indexes, adjacency references, visibility defaults. |
| `DoorState` | door ID, endpoint place IDs, open/closed state, optional locked state, optional key/access placeholder, whether closed door blocks movement. |
| `ContainerState` | container ID, physical location holder, open/closed state, optional locked state, contents ordered deterministically, capacity if implemented, visibility of contents when closed. |
| `ItemState` | item ID, portable flag, size/weight/carry cost if implemented, current holder/location, value-token metadata if applicable. |
| `OwnershipCustody` | legal owner actor/household placeholder, custodian actor/household placeholder, permitted access actors if needed, expected-location stub. No institution logic. |
| `Location` | one authoritative holder for each physical item/container: at place, in container, carried by actor, or equivalent. |
| `ControllerBinding` | non-world runtime metadata: controller ID, bound actor ID or none, mode, binding sequence. Stored outside authoritative physical world. |

### 12.3 Location model

Each physical item must have exactly one current location. Phase 1 must not allow an item to be simultaneously in a place, in a container, and carried by an actor.

Required holder variants:

| Holder | Meaning |
|---|---|
| `AtPlace(place_id)` | Item is directly in a place. |
| `InContainer(container_id)` | Item is inside a container. |
| `CarriedBy(actor_id)` | Item is carried by an actor. |

Containers themselves may be located at a place or, if Phase 1 chooses to allow portable containers, carried/in-container. The simplest acceptable Phase 1 implementation is to keep containers fixed at places. If portable containers are allowed, recursive containment must be cycle-checked and deterministic.

### 12.4 Visibility and reachability in Phase 1

Phase 1 embodied view is minimal physical locality, not full epistemics.

An actor may see or act on:

- the current place;
- doors connected to the current place;
- open containers in the current place;
- closed containers as objects, but not their hidden contents unless the container declares contents visible while closed;
- directly visible items in the current place;
- items carried by the actor;
- other actors in the same place if represented.

The actor may not see:

- contents of closed opaque containers;
- items in other places;
- debug truth;
- event-log truth;
- future Phase 2 belief facts;
- controller binding as a world fact.

### 12.5 Ownership/custody/access placeholders

Phase 1 must preserve the distinction among:

- legal owner;
- current physical holder;
- custodian/controller of access;
- expected location stub;
- permitted access.

However, Phase 1 must not implement legal procedure, suspicion, sanctions, or belief contradiction. Access may be used only as a physical validation fact if the fixture explicitly makes a door/container locked or permission-gated. If access denial is not implemented, ownership/custody still must validate and remain available to debug/future systems.

### 12.6 State mutation rule

Authoritative physical state may change only through world event application. Validation builds a mutation plan. Commit creates event envelopes. Event application changes state. Tests and TUI must use the same path.

---

## 13. Fixture/content validation requirements

Fixture validation is mandatory in Phase 1. Invalid content must fail before play.

### 13.1 Validation phases

The content validator must run deterministic validation in these phases:

1. **Parse/schema validation:** required fields, known sections, known versions, correct primitive types.
2. **Canonicalization:** stable path/entity ordering and normalized IDs.
3. **ID validation:** uniqueness, reserved words, stable ID syntax, no display-name-as-ID shortcuts.
4. **Referential validation:** every referenced actor/place/door/container/item/action exists and has the expected kind.
5. **Location validation:** every item has exactly one holder; container contents and item locations agree; no cycles; no duplicate contents; place membership/indexes agree.
6. **Physical topology validation:** door endpoints exist; adjacency is consistent; blocked movement has explicit door/edge state; no one-way edge unless explicitly supported and tested.
7. **State validation:** open/closed/locked values valid; locked objects have coherent access placeholders if lock behavior is used; carried items respect capacity if capacity exists.
8. **Action registry parity validation:** every fixture-declared affordance references a known action definition; every target kind is supported by the action definition; no unsupported target/action pairing.
9. **Semantic view validation:** action IDs expected by fixtures/view-model tests are stable semantic IDs, not menu positions.
10. **No-player validation:** no player-only verbs, player-only entities, player-only content tags, or player-privileged action definitions.
11. **No-script validation:** no authored outcome chains, quest triggers, forced event schedules, story beat conditions, reward objectives, culprit pointers, or UI-driven content triggers.
12. **Determinism hazard validation:** no outcome-affecting unordered lists/maps without canonical sorting or explicit ordering.
13. **Fixture contract validation:** each required golden fixture declares setup, allowed actions, expected events/reports, and acceptance assertions.
14. **Serialization validation:** accepted fixtures and event logs can be serialized/deserialized without changing canonical checksum.

### 13.2 Required validation failures

The validator must catch and test at least these failures:

| Failure | Required behavior |
|---|---|
| Missing stable ID | Fatal validation error naming path/entity. |
| Duplicate ID | Fatal deterministic error listing all duplicate locations. |
| Bad reference | Fatal error with source field and missing target ID. |
| Wrong target kind | Fatal error, e.g. action `open` targets an `Item` that is not openable. |
| Unsupported action target | Fatal error during content validation or action-registry parity validation. |
| Item double-location | Fatal error. |
| Container/item mismatch | Fatal error if container contents list disagrees with item holder. |
| Door endpoint missing | Fatal error. |
| Door/adjacency inconsistency | Fatal unless explicitly modeled and tested. |
| Non-deterministic ordering hazard | Fatal or canonicalized with deterministic warning promoted to test assertion. |
| Player-only verb | Fatal. |
| Player-only entity/tag | Fatal. |
| Quest/objective/reward content | Fatal. |
| Authored outcome chain | Fatal. |
| Fixture lacks expected golden assertions | Fatal for required golden fixtures. |

### 13.3 Forbidden content forms

The schema must reject ordinary content that contains structures equivalent to:

- `quest`, `objective`, `reward`, `player_character`, `player_only`, `protagonist`, `culprit`, `story_beat`, `drama_director`;
- `on_enter`, `on_open`, `on_tick`, `when_player`, `after_waits`, `force_event`, `spawn_event`, `complete_objective`, or any similar trigger that authors outcomes;
- commands that mutate state directly from content;
- TUI-specific action availability rules;
- LLM prompts as authoritative action, event, or fixture content.

The validator may allow display prose that includes natural words like “reward” only if those strings are inert labels and cannot create mechanics. Mechanical keys, tags, entity kinds, action definitions, and validation-affecting fields must be strict.

### 13.4 Required fixture serialization

Durable save-game UX is not required. Serialization for fixtures, event logs, validation reports, replay reports, and golden test artifacts is required.

Phase 1 must provide stable serialization sufficient to:

- commit golden fixtures to repository test data;
- commit expected event-log/replay artifacts or checksums;
- reproduce a failed validation report;
- run replay from serialized fixture plus serialized ordered events;
- snapshot TUI/view-model transcripts.

---

## 14. Command/proposal/action validation pipeline

All ordinary world actions must go through one shared path. This is the spine of Phase 1.

### 14.1 Inputs

Action proposals may originate from:

| Origin | Example | World privilege? |
|---|---|---:|
| Human controller through TUI | bound controller selects `open strongbox_tomas` | no |
| Test harness | test constructs same semantic proposal | no |
| No-human scheduler/process | process proposes `wait` or ordinary actor action | no |
| Future autonomous agent | actor proposes action from planner/routine | no |

The origin is metadata for debugging and provenance. It must not change physical validation rules.

### 14.2 Proposal data contract

An action proposal must include or resolve to:

| Field | Requirement |
|---|---|
| `proposal_id` | Stable deterministic ID within run/fixture. |
| `origin` | Human/test/scheduler/agent/debug metadata. Not world authority. |
| `actor_id` | Required for ordinary world actions. Must name an ordinary actor. |
| `action_id` | Stable action definition ID. |
| `target_ids` | Stable target IDs, semantically named or ordered by action definition. |
| `parameters` | Deterministic scalar parameters only, if any. No free-form LLM parsing. |
| `requested_tick` | Tick/order at which command was submitted. |
| `source_view_model_id` | Optional for TUI provenance. Must not be trusted as authority. |

### 14.3 Pipeline stages

The shared validation/commit pipeline must run these stages, even if some are trivial in Phase 1:

1. **Origin intake:** accept human/test/scheduler proposal and attach provenance metadata.
2. **Controller binding check:** only for human-origin commands, confirm a controller is bound to the submitted actor. This is command authorization, not world validation.
3. **Action definition lookup:** resolve `action_id` in the action registry.
4. **Actor lookup:** confirm actor exists and is enabled.
5. **Target binding:** resolve targets and verify expected target kinds.
6. **Locality/reachability validation:** confirm actor can physically interact with targets.
7. **Physical precondition validation:** door/container state, item holder, carrying state, capacity, locks/access if implemented.
8. **Phase boundary validation:** reject action definitions not implemented in Phase 1.
9. **Mutation plan construction:** produce canonical state delta plan for accepted actions.
10. **Validation report creation:** store accepted/rejected status, checked facts, reason codes, and stage results.
11. **Event envelope construction:** for accepted meaningful changes, create versioned world event(s); for required diagnostic rejections, create diagnostic `ActionRejected` event.
12. **Event append:** append events in deterministic order.
13. **Event application:** mutate state only by applying world events.
14. **Projection/view update:** rebuild or incrementally update projections from state/events.
15. **Debug linkage:** link proposal, validation report, event IDs, and resulting checksums.

### 14.4 Required Phase 1 ordinary action definitions

| Action ID | Actor required | Targets | Accepted effect | Event behavior |
|---|---:|---|---|---|
| `look` or `inspect_place` | yes | current place implicit or place ID | Produces view/query response only. | No world event in Phase 1. Validation report may be retained for debug. |
| `inspect_entity` | yes | visible/reachable item, container, door, actor, or place | Produces view/query response only. | No world event in Phase 1. Does not create knowledge. |
| `move` | yes | destination place ID, optional door/edge ID | Actor changes place if adjacent and not blocked. | `ActorMoved`. |
| `open` | yes | door or container ID | Target changes to open if reachable and closed/unlocked/access-valid. | `DoorOpened` or `ContainerOpened`. |
| `close` | yes | door or container ID | Target changes to closed if reachable and open. | `DoorClosed` or `ContainerClosed`. |
| `take` | yes | item ID plus source holder if needed | Item moves to actor/carrier if reachable and portable. | `ItemRemovedFromContainer` or `ItemTakenFromPlace`. |
| `place` | yes | item ID plus destination place/container | Item moves from actor/carrier to destination. | `ItemPlacedInContainer` or `ItemPlacedInPlace`. |
| `wait` | yes for actor wait; optional process variant for no-human | tick count or default one tick | Advances deterministic simulation time/order. | `ActorWaited` and/or `TimeAdvanced`. |

Debug/control commands such as controller attach, detach, switch, event-log inspection, item-location report, rejection report, projection rebuild, and replay are not ordinary world actions. They must not appear as actor affordances in embodied view models.

### 14.5 Rejection reports

Every rejected ordinary action must produce a structured validation report. The TUI must be able to display an actor-facing why-not message derived from the report, while debug can show the full structured report.

Required report fields:

| Field | Meaning |
|---|---|
| `validation_report_id` | Stable report ID. |
| `proposal_id` | Proposal being validated. |
| `actor_id` | Actor, if any. |
| `action_id` | Action definition. |
| `target_ids` | Targets resolved or attempted. |
| `status` | `accepted` or `rejected`. |
| `failed_stage` | First decisive failed stage. |
| `reason_codes` | Stable machine-readable reason codes. |
| `checked_facts` | Deterministically ordered list of material facts consulted. |
| `actor_visible_summary` | Safe why-not summary for embodied TUI. |
| `debug_summary` | Full non-diegetic explanation. |
| `would_mutate` | False for rejected actions. |
| `event_ids` | Events produced, if any. For rejected actions, only diagnostic events. |
| `checksum_before` | Physical checksum before validation or before commit. |
| `checksum_after` | Present for accepted world mutations. Must match event application. |

Required stable reason codes include at least:

- `unknown_action_id`;
- `phase_unsupported_action`;
- `controller_unbound`;
- `controller_actor_mismatch`;
- `actor_not_found`;
- `target_not_found`;
- `unsupported_target_kind`;
- `actor_not_at_required_place`;
- `not_adjacent`;
- `door_closed_blocks_movement`;
- `door_locked`;
- `container_closed`;
- `container_locked`;
- `target_not_visible`;
- `target_not_reachable`;
- `item_not_portable`;
- `item_not_at_source`;
- `item_not_carried`;
- `carry_capacity_exceeded`;
- `destination_not_open`;
- `already_open`;
- `already_closed`;
- `invalid_parameter`;
- `world_state_mismatch`.

### 14.6 Rejection versus failure

A **rejection** means the action did not pass validation and produced no world mutation. A **failure** means an accepted scheduled/durational action later failed during execution. Phase 1 should prefer immediate actions and rejections. If it introduces scheduled action starts, it must test `ActionStarted` and `ActionFailed` separately.

### 14.7 Action registry parity

The action registry is the single list of ordinary action definitions. The TUI view model, tests, no-human scheduler, and future AI must all use this registry.

Acceptance tests must prove:

- no action is available only to human-origin proposals;
- no TUI semantic action ID lacks a registry action definition;
- no registry action is implemented only in the TUI;
- non-human and human proposals with the same actor, action, targets, and state produce the same validation report status, reason codes, events, and checksum changes;
- debug/control commands are clearly outside the ordinary action registry and cannot be proposed as world actions by an actor.

---

## 15. TUI/view-model contract

Phase 1 must be playable through a TUI. It can be stark. It cannot be fake.

### 15.1 TUI responsibilities

The TUI must:

- bind a human controller to an actor through a debug/control command or startup selection;
- display the bound actor ID and current place ID;
- display local place description data from view models;
- display visible local doors, containers, items, actors, and exits;
- list available semantic actions with stable semantic IDs;
- allow selecting/submitting those semantic actions;
- display structured why-not output for rejected actions;
- allow waiting/advancing time;
- expose debug panels for item location, action rejection, event log, controller binding, projection rebuild, and replay report;
- preserve debug/embodied separation visually and structurally;
- support deterministic transcript or snapshot tests.

### 15.2 TUI non-responsibilities

The TUI must not:

- decide whether an action is valid;
- infer hidden item locations;
- mutate world state;
- apply events;
- rebuild projections using terminal-local state;
- reveal debug truth in embodied panels;
- parse natural language through an LLM;
- use terminal menu index as action identity;
- add player-only actions or player-only views.

### 15.3 Embodied local view model

An embodied view model must include at least:

| Field | Requirement |
|---|---|
| `view_model_id` | Stable enough for transcript/debug correlation. |
| `mode` | `embodied`. |
| `viewer_actor_id` | Bound actor. |
| `sim_tick` | Current deterministic tick. |
| `place_id` | Actor's current place. |
| `place_label` | Display label only. |
| `visible_exits` | Deterministically ordered exits/destinations with blockers summarized. |
| `visible_doors` | Deterministically ordered local doors and actor-visible state. |
| `visible_containers` | Local containers and actor-visible open/closed/locked state where allowed. |
| `visible_items` | Directly visible local/carried items. Hidden container contents excluded. |
| `local_actors` | Other visible actors in place, if represented. |
| `semantic_actions` | Deterministically ordered action affordance entries. |
| `last_rejection_summary` | Optional actor-safe why-not summary from last rejected proposal. |
| `debug_available` | Boolean or command hint. Must not embed debug truth. |

### 15.4 Semantic action entries

Each available or disabled action entry must include:

| Field | Requirement |
|---|---|
| `semantic_action_id` | Stable ID for selection. Must not be a menu index. |
| `action_id` | Registry action ID. |
| `target_ids` | Stable targets. |
| `label` | Human-readable display label. Not authority. |
| `enabled` | Whether proposal should pass known local affordance checks. Disabled entries may be shown for why-not UX. |
| `why_disabled` | Optional actor-safe reason summary or reason code. |

Examples of acceptable semantic action IDs:

- `move.to.place_kitchen`;
- `open.container.strongbox_tomas`;
- `close.door.door_tomas_bedroom_hall`;
- `take.item.coin_stack_01.from.strongbox_tomas`;
- `place.item.coin_stack_01.in.strongbox_tomas`;
- `inspect.container.strongbox_tomas`;
- `wait.1_tick`.

These examples are illustrative; the implementation may choose a different stable naming convention. It must be deterministic, target-specific, and testable.

### 15.5 Debug view models

Debug view models must be separate from embodied view models. Required debug panels:

| Debug panel | Required output |
|---|---|
| Controller binding | Current controller binding metadata, bound actor if any, binding history from controller stream. |
| Event log | Ordered event envelopes or summaries with stream, position, type, actor/process, participants, causes, and checksum where present. |
| Item location | Structured item-location debug report. |
| Action rejection | Structured validation/rejection report for selected proposal/report ID. |
| Projection rebuild | Rebuild report from fixture plus ordered events. |
| Replay report | Replay checksum/report and diff if any. |

Debug panels must be visually marked as debug/non-diegetic. Tests must prove opening debug does not change embodied view facts except for visible UI mode state outside world simulation.

### 15.6 TUI tests

Phase 1 must include both semantic view-model tests and at least one transcript or snapshot test.

Required coverage:

- initial bind to an actor;
- render local view;
- list stable semantic actions;
- select `open`, `take`, `place`, `move`, `wait`, and a rejected action through semantic IDs;
- display why-not for a rejected action;
- open debug item-location report;
- rebuild projection/replay from TUI-accessible debug path or equivalent harness;
- verify the TUI never mutates state directly.

---

## 16. Controller binding and possession parity requirements

### 16.1 Controller binding model

A controller binding must be represented outside authoritative world state. A minimal binding record includes:

| Field | Meaning |
|---|---|
| `controller_id` | Runtime human/debug controller identity. |
| `bound_actor_id` | Actor currently receiving human commands, or none. |
| `mode` | Embodied, debug, or unbound/control mode. |
| `binding_sequence` | Deterministic local sequence for debug/control stream. |
| `created_at_tick` | Simulation tick for correlation only. Does not become a world fact. |

The binding may be logged in the `controller` stream as `ControllerAttached`/`ControllerDetached` events. Those events are not world events and must not be applied to physical state.

### 16.2 No knowledge transfer

Changing controller binding must not:

- alter any actor component;
- create an observation;
- create a memory;
- reveal debug truth to the actor;
- mark a target as known;
- create social suspicion;
- change action preconditions;
- change local visibility except by choosing a different actor's embodied view.

Phase 1 has minimal embodied physical views, not full belief stores, but it still must prove the negative: debug truth and controller binding metadata do not enter the world or actor state.

### 16.3 Shared action path

Human commands and non-human proposals must use the same action validation/commit path after command authorization.

The only human-specific check is: “is this controller currently bound to this actor?” Once a valid actor/action/target proposal exists, all ordinary validation and commit behavior is identical.

### 16.4 Possession parity tests

Required tests:

1. Same actor, same action, same target, same state, human origin versus non-human/test origin produces same accepted result.
2. Same actor, same invalid action, same state, human origin versus non-human/test origin produces same rejection reason codes after controller authorization.
3. Changing controller binding creates no world event and no physical checksum change.
4. Human-bound actor has no extra ordinary semantic actions compared with the same actor under non-human proposal enumeration.
5. Fixture/content validation rejects player-only verbs and player-only tags.
6. No-human advance can run with no controller binding at all.

---

## 17. Debug/provenance requirements

Debug is not optional. Phase 1 must let developers and tests inspect cause without corrupting play.

### 17.1 Item-location debug report

The item-location report must answer: where is this item, and what caused it to be there?

Required fields:

| Field | Meaning |
|---|---|
| `report_id` | Stable debug report ID. |
| `item_id` | Queried item. |
| `exists` | Whether item exists. |
| `current_location` | Holder chain, e.g. item in container at place, or carried by actor at place. |
| `location_chain` | Deterministically ordered chain from item to place/actor/container context. |
| `current_projection_position` | Event stream position/projection version used. |
| `last_location_event_id` | Last world event that changed item location, or fixture-origin marker. |
| `location_event_chain` | Ordered relevant event IDs from fixture origin through moves. |
| `relevant_events` | Summaries of take/place/remove events. |
| `inconsistencies` | Empty for passing state; otherwise validation/debug warnings. |
| `physical_checksum` | Current physical checksum used for report. |
| `debug_only` | Explicit marker that this report is non-diegetic. |

The report must handle fixture-origin location before any movement and event-derived location after movement.

### 17.2 Action-rejection debug report

The action-rejection report must answer: why did this proposal fail, and what state facts caused that rejection?

Required fields:

| Field | Meaning |
|---|---|
| `report_id` | Stable debug report ID. |
| `proposal_id` | Failed proposal. |
| `validation_report_id` | Underlying validation report. |
| `failed_stage` | Decisive pipeline stage. |
| `reason_codes` | Stable machine-readable reasons. |
| `actor_visible_summary` | Safe why-not text. |
| `debug_summary` | Full non-diegetic explanation. |
| `checked_facts` | State facts consulted, with entity IDs and values. |
| `precondition_trace` | Ordered validation steps leading to rejection. |
| `events_created` | Diagnostic events only; no world events. |
| `mutation_attempted` | Must be false for validation rejection. |
| `checksum_before` | Physical checksum before rejection. |
| `checksum_after` | Must match before for rejection. |
| `debug_only` | Explicit non-diegetic marker. |

### 17.3 Projection rebuild debug report

Required fields:

- report ID;
- fixture ID;
- content manifest ID;
- initial checksum;
- event count;
- applied event IDs/positions;
- final checksum;
- projection IDs rebuilt;
- errors/diffs/invariant violations;
- debug-only marker.

### 17.4 Replay debug report

Replay report fields are defined in Section 11.7. The debug UI must be able to display this report or invoke the same core API used by automated replay tests.

### 17.5 Debug leakage regression

At least one regression test must prove:

1. an embodied view does not include hidden item location;
2. debug item-location report reveals item location;
3. returning to embodied view still does not include hidden item location unless physically visible by Phase 1 rules;
4. physical checksum and actor state are unchanged by debug inspection;
5. controller binding metadata remains outside world state.

---

## 18. Golden fixture suite

The seven Phase 1 golden fixtures are mandatory. They may be small. They must be executable and serialized. Each fixture must load deterministically, validate cleanly, run its scenario, and pass replay/debug/TUI assertions where applicable.

### 18.1 `strongbox_001`

**Purpose:** Physical strongbox/value-token/custody baseline inherited from Spec 0001, narrowed to Phase 1 physical kernel.

**Minimum setup:**

- `actor_tomas` exists as an ordinary actor.
- At least one other ordinary actor exists, preferably `actor_elena` if following Spec 0001 fixture names.
- `house_tomas` or equivalent place group exists.
- A local place containing `strongbox_tomas` exists.
- `strongbox_tomas` is a container with deterministic ID and open/closed state.
- `coin_stack_01` is a physical item/value token initially located in `strongbox_tomas`.
- Ownership/custody/access fields distinguish owner, current holder, custodian, and expected-location stub.
- A door/local movement edge exists if needed to prove local access.

**Required scenario assertions:**

- fixture validation accepts stable IDs and physical relationships;
- `coin_stack_01` is not an abstract balance;
- debug item-location report identifies fixture-origin location inside `strongbox_tomas`;
- opening `strongbox_tomas` through an ordinary actor action produces `ContainerOpened` if valid;
- taking `coin_stack_01` through an ordinary actor action produces an item-location world event;
- replay rebuild produces the same final item location and checksum;
- no belief contradiction, suspicion, report, or institutional logic is asserted.

**Required negative assertions:**

- no quest/objective/reward/player content;
- no forced theft or forced discovery;
- no actor learns debug truth;
- no outcome script moves the coins.

### 18.2 `container_item_move_001`

**Purpose:** Prove open/take/place event order and projection rebuild for item movement.

**Minimum setup:**

- one actor in a place;
- one closed or open container in that place;
- one portable item in the container;
- optional second container or direct place destination.

**Required action path:**

1. bind controller or use test proposal for ordinary actor;
2. inspect local view;
3. open container if closed;
4. take item from container;
5. place item into destination container or place;
6. rebuild projection;
7. replay event log.

**Required events/reports:**

- `ContainerOpened` if initially closed;
- `ItemRemovedFromContainer`;
- `ItemPlacedInContainer` or `ItemPlacedInPlace`;
- validation reports for accepted actions;
- replay report with matching checksum.

**Required rejection case:**

Attempt to take an item from a closed container or from the wrong holder must produce a structured rejection and no world mutation.

### 18.3 `door_access_001`

**Purpose:** Prove local movement, adjacency, door blockers, open/close events, and why-not.

**Minimum setup:**

- two places connected by a door;
- one actor on one side;
- the door initially closed and configured to block movement while closed.

**Required action path:**

1. attempt to move through the closed blocking door;
2. receive rejection `door_closed_blocks_movement` or equivalent;
3. open door;
4. move to adjacent place;
5. optionally close door from reachable side;
6. replay.

**Required events/reports:**

- diagnostic `ActionRejected` or retained validation report for blocked movement;
- `DoorOpened`;
- `ActorMoved`;
- `DoorClosed` if close is included;
- matching replay checksum/report.

### 18.4 `debug_attach_001`

**Purpose:** Prove controller binding is non-world metadata and debug attach/switch has no player privilege.

**Minimum setup:**

- at least two ordinary actors in deterministic initial places;
- no required world events before binding.

**Required action path:**

1. start unbound or bind to actor A;
2. render embodied view for actor A;
3. switch/debug attach to actor B;
4. render embodied view for actor B;
5. compare physical checksum before/after binding changes;
6. attempt ordinary action through same shared pipeline.

**Required assertions:**

- `ControllerAttached` appears only in controller stream or equivalent run metadata;
- physical checksum does not change due to attach/switch;
- actor state does not gain knowledge or flags;
- bound actor's available ordinary actions are the same actions a non-human proposal could use;
- debug view is explicitly non-diegetic.

### 18.5 `no_human_advance_001`

**Purpose:** Prove the simulation can advance with no sacred player state.

**Minimum setup:**

- at least one ordinary actor;
- no controller binding required;
- deterministic scheduler/time state.

**Required action path:**

1. load fixture;
2. run advance for a fixed tick count with no controller binding;
3. produce deterministic events/reports;
4. replay.

**Required assertions:**

- no `PlayerCharacter`, player objective, player-bound scheduler priority, or human-only branch;
- no-human advance produces `NoHumanAdvanceStarted` and completion/report marker or equivalent deterministic process report;
- if time advances, it does so through deterministic tick/event machinery;
- replay report matches expected checksum;
- the same kernel APIs are used as TUI/test action execution where ordinary actions occur.

Phase 1 does not need routines or autonomous social life. A no-human no-op/time-advance proof is acceptable only if it proves there is no controller/player dependency and the event/replay machinery is real. If the fixture includes an ordinary scheduled action, it must use the same action pipeline.

### 18.6 `replay_item_location_001`

**Purpose:** Prove item location derives from initial fixture plus ordered events, not mutable current-state accident.

**Minimum setup:**

- one item with fixture-origin location;
- at least one valid item movement event sequence.

**Required action path:**

1. load fixture;
2. move item through take/place actions;
3. record ordered events;
4. rebuild from initial fixture plus events;
5. ask item-location debug report on live state and replayed state;
6. compare checksum/report.

**Required assertions:**

- final item holder/location matches live run;
- item-location report names last location-changing event;
- projection rebuild event count and checksum match;
- deleting/reordering an item movement event causes replay report failure, not silent repair.

### 18.7 `view_model_local_actions_001`

**Purpose:** Prove the TUI/view-model exposes stable semantic local actions without UI rule ownership.

**Minimum setup:**

- one actor in a place with at least one exit, one door or container, and one item interaction path;
- at least one action that is enabled and one action that is disabled/rejected for why-not.

**Required action path:**

1. bind controller to actor;
2. build embodied view model;
3. assert deterministic action list order;
4. select semantic action by stable ID;
5. submit action through shared pipeline;
6. render updated view model;
7. submit invalid semantic action or disabled action;
8. display why-not;
9. open debug panel without leaking debug truth into embodied view.

**Required assertions:**

- view model action IDs are stable and target-specific;
- terminal menu order is not action identity;
- TUI does not mutate state;
- why-not derives from validation report;
- debug and embodied view models are separate;
- transcript/snapshot is deterministic.

---

## 19. Testing matrix and acceptance gates

### 19.1 Required automated test classes

Phase 1 must include automated tests for every row below.

| Test class | Required assertions |
|---|---|
| Deterministic fixture loading | Same fixture loads to same canonical state/checksum across repeated loads; entity order stable. |
| Content/schema validation | Required fields, IDs, references, target kinds, locations, no-player, no-quest, no-script, and determinism hazards fail deterministically. |
| Action registry parity | Human/test/no-human proposals use same action definitions and validation path; no player-only verbs. |
| Physical precondition validation | Movement adjacency, door blockers, container open/closed/locked state, item source, carrying, destination, and target reachability. |
| Action rejection reports | Stable reason codes, failed stage, checked facts, no mutation, actor-safe why-not, debug report. |
| Event envelope versioning | Every event has schema version; unknown version fails replay; event kind registry tested. |
| Event append order | Accepted meaningful actions append events in deterministic stream order. |
| Event application | Events apply strictly; impossible old state fails loudly. |
| Projection rebuild | Rebuild from initial fixture plus events matches live projection/state. |
| Replay checksum/report | Replay produces matching checksum/report; drift fails. |
| No-human advance | Runs without controller/player; deterministic report/events. |
| No player-only action path | TUI, tests, and non-human proposals cannot find a player-only world branch. |
| Controller metadata separation | Binding changes affect only controller stream/runtime metadata, not physical checksum or actor knowledge/state. |
| TUI/view-model semantic action selection | View model exposes stable action IDs; TUI submits those IDs; no direct mutation. |
| TUI transcript/snapshot | Representative play transcript is deterministic and covers view, action, rejection, wait, debug. |
| Golden fixture scenarios | All seven required fixtures pass their contracts. |
| Debug truth non-leakage | Debug item/rejection truth does not enter embodied state. |
| LLM-disabled behavior | All tests pass with no LLM dependency. |
| No authored outcome chain | Content scripts/triggers/objectives are rejected. |

### 19.2 Acceptance gates

Phase 1 passes only if all of these gates pass:

1. The seven required Phase 1 fixtures load deterministically.
2. Fixture validation catches missing IDs, duplicate IDs, bad references, unsupported action targets, non-deterministic ordering hazards, and player-only verbs.
3. Fixture validation rejects quest/objective/reward/player/protagonist/content-script constructs.
4. Actions are validated through one shared path regardless of human or non-human origin.
5. Invalid actions produce structured rejections with stable reason codes.
6. Meaningful accepted actions append versioned events.
7. Physical state mutates only through event application.
8. Projections rebuild from fixture plus ordered events.
9. Replay produces matching physical state and checksum/report.
10. Item location can be explained by structured debug report.
11. Action rejection cause can be explained by structured debug report.
12. The TUI can bind to an actor, show a local view, list semantic actions, submit actions, show why-not, wait/advance, and access debug view.
13. No-human advance runs with no sacred player entity, no player-only branch, and no controller requirement.
14. Possession/controller binding creates no world fact and no knowledge transfer.
15. `strongbox_001` passes as a Phase 1 physical/event/replay fixture without faking Phase 2 beliefs.
16. `container_item_move_001` passes.
17. `door_access_001` passes.
18. `debug_attach_001` passes.
19. `no_human_advance_001` passes.
20. `replay_item_location_001` passes.
21. `view_model_local_actions_001` passes.
22. No LLM is required for Phase 1 behavior.
23. No authored outcome chain is accepted as ordinary content.

A failure in any gate is a Phase 1 failure, not a known limitation.

### 19.3 Minimum regression suite names

The implementation should include test names equivalent to these, even if exact naming differs:

- `loads_fixtures_deterministically`;
- `rejects_missing_ids_and_bad_references`;
- `rejects_unsupported_action_targets`;
- `rejects_nondeterministic_ordering_hazards`;
- `rejects_player_only_verbs`;
- `rejects_quest_and_script_content`;
- `human_and_nonhuman_proposals_share_validation_path`;
- `blocked_movement_returns_structured_rejection`;
- `closed_container_take_returns_structured_rejection`;
- `accepted_actions_append_versioned_events`;
- `event_append_order_is_deterministic`;
- `projection_rebuild_matches_live_state`;
- `replay_checksum_matches`;
- `replay_detects_missing_or_reordered_event`;
- `debug_item_location_reports_last_location_event`;
- `debug_rejection_report_names_failed_stage`;
- `controller_binding_is_not_world_state`;
- `no_human_advance_requires_no_controller`;
- `tui_selects_semantic_action_id_not_menu_index`;
- `tui_transcript_is_deterministic`;
- `debug_truth_does_not_enter_embodied_view`;
- `llm_disabled_phase1_still_passes`.

---

## 20. Implementation constraints and anti-patterns

### 20.1 Hard implementation constraints

- Keep Phase 1 small. Build the serious physical/event/TUI/replay spine, not a general life-sim platform.
- Prefer typed records and explicit maps over a broad ECS unless the ECS is genuinely smaller and easier to validate/replay.
- Use ordered collections or explicit sorted iteration for outcome-affecting logic.
- Do not rely on Rust `HashMap` or `HashSet` iteration order for outcomes, event order, view-model order, validation error order, or checksums.
- Version event envelopes from the first committed event.
- Keep content validation strict. Rejection is better than silently accepting shape drift.
- Keep debug truth out of embodied state.
- Keep controller binding out of world state.
- Keep the TUI dumb about rules.
- Keep replay strict and loud.

### 20.2 Forbidden shortcuts

The following shortcuts are forbidden because they would invalidate the product proof:

| Shortcut | Why forbidden |
|---|---|
| `PlayerCharacter` entity | Violates ordinary-agent possession and no sacred player. |
| Player inventory | Creates protagonist privilege and bypasses physical item model. |
| TUI directly mutates state | Bypasses kernel, events, replay, validation, and no-human parity. |
| Debug command used as ordinary action | Confuses non-diegetic tooling with world causality. |
| Save current state instead of replay | Avoids the event-sourced proof. |
| Event log without schema versions | Creates replay/schema debt at the first event. |
| Unstructured why-not strings only | Cannot support debug provenance, tests, or stable UI. |
| Hash iteration for event/view order | Breaks determinism. |
| Fixture scripts outcomes | Turns Tracewake into authored scenario playback. |
| LLM interpretation of commands | Adds nondeterministic authority and violates LLM-disabled gate. |
| Hidden player-only branch | Destroys possession parity. |
| Debug truth inserted into actor view | Destroys epistemic separation before Phase 2 exists. |
| Broad ECS/framework first | Likely hides rules and delays the actual proof. |
| Branch/default repository fetches for specs/tests | Violates exact-source discipline for implementation planning. |

### 20.3 Strong guidance for the coding agent

Implement the vertical slice first. A small, boring kernel that deterministically opens a strongbox, moves coins, explains why a closed door blocks movement, and replays to a checksum is the right Phase 1. A grand architecture with no playable TUI and no replay report is the wrong product.

Do not add hunger because actors exist. Do not add reports because the strongbox exists. Do not add belief because inspect exists. Do not add LLM text because the TUI is ugly. Do not add quests because a human needs something to do. The thing to do in Phase 1 is inspect causality.

---

## 21. Phase 1 exit criteria

Phase 1 exits only when the repository contains an implementation that satisfies all criteria below.

### 21.1 Build/run criteria

- Workspace builds with the compact Rust shape or a documented smaller equivalent.
- TUI executable can load at least one Phase 1 fixture.
- Fixture validation can be run automatically.
- Golden tests can be run automatically.
- Replay/checksum tooling can be run automatically.

### 21.2 Play criteria

Through the TUI, a user can:

- bind to an ordinary actor;
- see the actor's local place;
- see local exits, doors, containers, visible items, and semantic actions;
- move between adjacent places when valid;
- see why movement is blocked when invalid;
- inspect local objects;
- open and close doors/containers when valid;
- take and place physical items when valid;
- wait/advance time;
- see structured why-not for invalid actions;
- open debug views for event log, item location, rejection report, projection rebuild, replay report, and controller binding.

### 21.3 Causal/replay criteria

- Every meaningful physical accepted action appends one or more versioned world events.
- Rejected actions produce structured reports and required diagnostic events without mutating world state.
- Event application is the only physical mutation path.
- Projection rebuild from fixture plus ordered events matches live physical state.
- Replay checksum/report matches for all eventful golden scenarios.
- Replay detects missing, reordered, unknown-version, or inconsistent events.

### 21.4 Possession/no-human criteria

- Controller binding is outside physical world state.
- Controller attach/switch/detach changes no physical checksum and no actor knowledge/state.
- Human and non-human ordinary proposals share validation and commit path.
- No-human advance runs with no controller binding and no player dependency.
- Fixture/content validation rejects player-only verbs and player-shaped content.

### 21.5 Validation/content criteria

- All seven required fixtures validate and pass.
- Negative validation fixtures prove missing IDs, duplicate IDs, bad references, unsupported target kinds, nondeterministic ordering hazards, player-only verbs, quest/objective/reward constructs, and authored outcome chains are rejected.
- Test fixture and event-log serialization is deterministic.

### 21.6 Debug criteria

- Item-location report explains fixture-origin and event-derived item location.
- Rejection report explains failed stage and state facts.
- Projection rebuild/replay reports are structured and deterministic.
- Debug truth non-leakage regression passes.

### 21.7 Phase boundary criteria

- No full beliefs.
- No expectation contradiction.
- No routines/needs/jobs/social life.
- No institutions/reports/suspicion/sanctions.
- No rumors/testimony/notices/travel beyond local adjacent movement.
- No LLM authority.
- No authored outcome chains.
- No graphical client requirement.
- No durable save-game UX beyond replay/test artifacts.

---

## 22. Deferred-to-Phase-2+ ledger

The following systems are deliberately deferred. They must not be implemented in Phase 1 except as inert placeholders explicitly described above.

| Deferred area | Earliest appropriate phase | Phase 1 treatment |
|---|---:|---|
| Full belief stores | Phase 2 | Do not implement. Preserve fields/placeholders only if needed. |
| Observation interpretation | Phase 2 | Do not implement. Inspect is a query/view action only. |
| Expectation contradiction | Phase 2 | Do not fake. `strongbox_001` only proves physical state/replay. |
| Actor notebooks | Phase 2+ | Placeholder only if needed; no active notebook behavior. |
| Actor-specific hidden epistemic filtering | Phase 2 | Phase 1 has only physical local visibility and debug separation. |
| Possession parity with beliefs | Phase 2 | Phase 1 proves action-path and debug non-leakage parity only. |
| Hunger/fatigue/safety/sleep/needs | Phase 3 | Do not implement. |
| Jobs/work/routines/planner cognition | Phase 3 | Do not implement. No-human may be no-op/time/process proof. |
| Autonomous social life | Phase 3+ | Do not implement. |
| Institutions/households/norms | Phase 4 | Inert placeholders only if required by fixture shape. |
| Reports/records/ledgers as active systems | Phase 4 | No active report or truth-making record behavior. |
| Suspicion/questioning/sanctions/wrong suspicion | Phase 4 | Do not implement. |
| Rumors/testimony/notices/stale leads | Phase 4+ / second proof | Do not implement. |
| Travel beyond local adjacent movement | Second proof | Do not implement. Local place graph only. |
| Regional simulation/LOD | Later | Do not implement. |
| LLM speech/parsing/prose | Later, if ever | No authority; disabled in Phase 1. |
| Graphical client | Later | TUI only. |
| Combat/magic/quests | Not part of Phase 1 product | Do not implement. Quest-shaped systems are rejected. |
| Durable save-game UX | Later | Serialization only for fixtures/event logs/replay/test artifacts. |
| Broad ECS/framework architecture | Only if later justified | Avoid in Phase 1 unless demonstrably smaller and safer. |

---

## 23. Final implementation-grade summary

Phase 1 is a narrow, ruthless proof of physical causality:

- stable content loads deterministically;
- ordinary actors perform ordinary local physical actions;
- human and non-human proposals use one validation path;
- invalid actions explain themselves structurally;
- accepted physical changes become versioned events;
- state rebuilds from fixture plus events;
- replay reports match;
- debug explains item location and rejection causes;
- TUI play is real and semantic;
- no-human advance runs without a player;
- possession is metadata, not world truth;
- the future missing-property proof remains possible because Phase 1 refuses to fake beliefs, institutions, scripts, or narrative outcomes.

The correct Phase 1 implementation is small, deterministic, playable, inspectable, and replayable. Anything broader that weakens those properties is a regression, not progress.
