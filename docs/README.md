# Tracewake Documentation

Tracewake's documentation is organized as layered authority. Read earlier layers before later layers.

```text
0-foundation/    constitutional doctrine
1-architecture/  subsystem architecture contracts
2-execution/     implementation and validation planning
3-reference/     compact lookup and active watchlists
4-specs/         subordinate corrective or implementation specs, when present
```

Foundation outranks architecture. Architecture translates foundation into subsystem contracts. Execution and specs are subordinate to both. Reference documents are lookup aids and risk registers; they do not create new doctrine.

If execution conflicts with architecture or foundation, execution is wrong. If implementation is more convenient than the accepted gates, implementation is wrong. If a test proves only plausible behavior while bypassing actor-known provenance, the test is wrong.

## Authority order

| Layer | Role | May define | May not define |
|---|---|---|---|
| `0-foundation/` | Constitutional identity and invariants | What Tracewake is, what it must never become, first playable proof, truth-firewall doctrine | Implementation shortcuts, phase-local exceptions, player privilege |
| `1-architecture/` | Stable subsystem contracts | Boundaries, data flows, authority ownership, diagnostics, acceptance implications | Tickets, code tasks, speculative implementation schedules |
| `2-execution/` | Work sequencing and proof planning | Phase gates, fixture expectations, validation plans | New doctrine that weakens foundation or architecture |
| `3-reference/` | Compact lookup and risk control | Glossary, checklist, risk register | Essays, hidden policy, implementation plans |
| `4-specs/` | Subordinate corrective or implementation specs | Narrow implementation direction under existing doctrine | Architecture replacement, constitutional changes, acceptance weakening |

The architecture layer has been corrected against the downstream staleness report. It now treats the holder-known context / truth-firewall split as a system-wide architecture spine, not a late Phase 3A clarification. This replacement set creates no tickets, no execution specs, and no implementation task breakdown.

## 0-foundation/ — constitutional layer

The foundation layer defines what Tracewake is and what it must not become.

- `00_FOUNDATION_INDEX.md` — map, authority, reading order, and anti-drift rules.
- `01_PROJECT_CHARTER.md` — product identity, Rust-first simulation, TUI-first playability, ordinary-life proof, and long-term posture.
- `02_CONSTITUTIONAL_INVARIANTS.md` — compact non-negotiable rules later layers must satisfy.
- `03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` — event authority, replay, traceability, snapshots, randomness, and forensic causality.
- `04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — typed claims/propositions, beliefs, observations, testimony, rumors, records, and knowledge flow.
- `05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — symbolic agents, BDI separation, durable intentions, HTN procedures, bounded local planning, and debug traces.
- `06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md` — action parity, affordances, ordinary life, survival substrate, search, storage, work, travel, and basic economy.
- `07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md` — households, roles, norms, records, notices, ownership/custody/access, fallible institutions, and future organizations.
- `08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — TUI-first playability, actor-filtered view models, possession, notebooks, why-not explanations, and debug quarantine.
- `09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md` — causal authoring, forbidden outcome chains, seeds, prehistory, records, notices, and no director logic.
- `10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md` — multi-resolution simulation, promotion/demotion, regional processes, long history, and scale posture.
- `11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md` — structured speech acts, optional LLM rendering/parsing, prompt boundaries, validation, and LLM-disabled operation.
- `12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md` — first village scope, missing-property proof, no-human gates, replay gates, TUI gates, and canonical regression seeds.
- `13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` — research decisions recorded as foundation constraints.
- `14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — constitutional truth firewall: truth may validate, but truth may not plan.

## 1-architecture/ — subsystem contract layer

The architecture layer translates foundation doctrine into subsystem contracts. The whole `docs/1-architecture/` directory should be treated as a complete replacement set. Architecture documents not listed here are retired and should not remain as live doctrine.

- `00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — architecture map, replacement/retirement rule, universal conformance questions, and maintenance rules.
- `01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` — Rust-first authority boundaries, workspace seams, dependency direction, content/domain-pack authority, and forbidden inversions.
- `02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` — event log, replay, projections, save manifests, random stream discipline, schema versioning, and migration failure.
- `03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — holder-known contexts, truth firewall, provenance packets, context sealing, debug comparison, and contamination gates.
- `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` — ordinary action proposal pipeline, scheduler limits, validation truth, failure semantics, affordances, and modeled feedback.
- `05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md` — canonical actor decision transaction, needs, commitments, intentions, routines, HTN method selection, local planning, and stuck diagnostics.
- `06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` — typed propositions, observations, beliefs, memories, records, rumors, leads, traces, and source-bound information flow.
- `07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md` — structured speech acts, speaker/listener contexts, language rendering/parsing, and optional LLM boundary.
- `08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md` — institution-known transaction, households, norms, roles, proof, records, notices, orders, sanctions, and procedural fallibility.
- `09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md` — missing-property first proof, settlement ontology, spaces, property, food, sleep, work, local economy, and road/adventure deferral.
- `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — possession, embodied TUI, actor-legible failure views, debug-only truth, view-model tests, and client boundaries.
- `11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md` — questless incidents, leads, notices, reports, observer-only story sifting, and second-proof boundaries.
- `12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md` — LOD as ontology, aggregate truth, promotion/demotion, regional processes, ancestry, and prehistory.
- `13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — acceptance artifacts, anti-contamination tests, no-human/TUI/replay gates, diagnostics, and review checklists.
- `14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md` — research distilled into decisions, consequences, and forbidden misreads.

## 2-execution/ — implementation and validation planning layer

Execution documents define the order of work, phase gates, first proof, fixtures, validation expectations, and what remains deferred. They do not replace foundation doctrine or architecture contracts. Some existing execution documents may still contain older architecture names or phase-local language; read them through the corrected architecture map until they are separately aligned.

## 3-reference/ — compact lookup and watchlist layer

Reference documents should remain compact. Use them to preserve terminology, checklist discipline, and design risks. Do not use them for architecture replacement, implementation plans, or new doctrine.

## Maintenance rules

When adding or changing documents:

- preserve authority order: foundation before architecture before execution/specs before reference;
- restart numbering inside each folder rather than continuing numbers across tiers;
- avoid stale filenames in indexes, examples, and reports;
- treat every world-affecting human command as an ordinary actor/process proposal under equivalent conditions;
- require holder-known provenance before cognition, procedure decisions, speech interpretation, view-model affordance selection, leads, institution procedure, or LOD promotion;
- keep debug truth visibly non-diegetic and structurally quarantined;
- keep reference documents short enough to serve as lookup tools;
- prefer typed claims, typed diagnostics, and replayable records over display strings.

Tracewake's recurring test is simple: every feature must preserve event-sourced causality, subjective epistemics, ordinary agents, possession parity, fallible institutions, questless leads, TUI-first playability, validation/replay, and no simulation fact born from prose.
