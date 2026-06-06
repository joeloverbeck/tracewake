# Execution Index and Authority

## Status

This folder defines the execution contract for the first implementable Tracewake proof.

Execution documents define phase order, phase gates, fixture contracts, validation contracts, debug and observability expectations, data-authoring boundaries, first-proof limits, and second-proof deferrals.

They do **not** define:

- implementation tickets;
- sprint plans;
- Rust source code;
- final crate choices;
- final UI polish;
- repository-audit evidence ledgers;
- connector histories;
- one-off migration notes.

Repository provenance for a review session belongs in that review artifact. It must not be embedded in committed execution authority documents.

## Authority relationship

Execution obeys this authority order:

```text
foundation doctrine
-> architecture contracts
-> execution phase gates and fixtures
-> implementation specs/code
-> tests and validation reports
```

When execution conflicts with foundation or architecture, execution is wrong.
When later implementation is more convenient than the execution gates, the implementation is wrong.
When a test rewards a shortcut that violates foundation doctrine, the test is wrong.

Execution may decide:

- which proof is first;
- which phase proves which behavior;
- which fixtures are mandatory;
- which acceptance artifacts block progression;
- which features remain deferred.

Execution may not weaken:

- causality before drama;
- belief before truth;
- ordinary life before adventure;
- no sacred player entity;
- no quest ontology;
- no authored outcome chains;
- no hidden director;
- no omniscient agent cognition;
- no LLM simulation authority;
- no abstract property model when physical custody is required;
- TUI-first playability;
- no-human simulation;
- event sourcing, deterministic replay, and forensic provenance.

## Execution document map

Read in order.

| File | Contract owned |
|---|---|
| `00_EXECUTION_INDEX_AND_AUTHORITY.md` | Folder map, authority, execution discipline, and maintenance rules. |
| `01_EXECUTION_CHARTER_SCOPE_AND_FIRST_PROOF.md` | First-proof charter, scope limits, narrowing decisions, and second-proof boundary. |
| `02_PHASE_LADDER_AND_ACCEPTANCE_GATES.md` | Strict phase ladder, progression rules, and global acceptance formula. |
| `03_FIRST_PROOF_MISSING_EXPECTED_PROPERTY.md` | Detailed first-proof scenario, fixture requirements, and required causal chain. |
| `04_PHASE_0_PAPER_ONTOLOGY_AND_FIXTURE_CONTRACTS.md` | Paper ontology, fixture design, event/action/proposition vocabulary, and anti-scripting review. |
| `05_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY.md` | Runnable kernel, TUI, physical actions, event log, replay, projection rebuild, and no-human seed. |
| `06_PHASE_2_EPISTEMICS_VIEW_MODELS_AND_POSSESSION_PARITY.md` | Beliefs, observations, memory, actor-filtered views, expectation contradiction, and possession parity. |
| `07_PHASE_3_NEEDS_ROUTINES_AND_NO_HUMAN_LIFE.md` | Needs, routines, ordinary life, autonomous theft opportunity, interruption, and no-human daily simulation. |
| `08_PHASE_4_INSTITUTIONS_RECORDS_AND_WRONG_SUSPICION.md` | Report intake, records, clerks/authority, partial testimony, procedural validity, and wrong suspicion. |
| `09_DATA_AUTHORING_VALIDATION_AND_GOLDEN_FIXTURES.md` | Domain packs, schema checks, fixture packs, golden scenarios, and content-validation discipline. |
| `10_TESTING_DEBUGGING_OBSERVABILITY_AND_METRICS.md` | Replay proof, causality inspection, no-human proof, view-model checks, provenance review, and metrics. |
| `11_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_AND_REGIONAL_EXPANSION.md` | Deferred second proof: notices, roads, travel, companions, threats, regional incidents, and expansion gates. |
| `12_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` | Stable execution research decisions and source-handling policy. |

## Strict phase posture

Tracewake execution uses a ladder, not a backlog heap.

A later phase may not be used to paper over a failed earlier phase. A broader feature may not compensate for a missing first-proof behavior. The first proof must be earned through ordinary, replayable, actor-grounded causes.

The minimum ladder is:

```text
Phase 0: paper ontology and fixture contracts
Phase 1: runnable kernel, TUI, event log, replay, physical property
Phase 2: beliefs, view models, expectation contradiction, possession parity
Phase 3: needs, routines, autonomous ordinary life, no-human operation
Phase 4: institutions, records, report intake, wrong suspicion
Second proof: notices, travel, regional expansion, and longer causal chains
```

## Universal phase gate

A phase is not accepted because code exists or a demo looks plausible.

A phase is accepted only when it has:

1. a playable TUI path for that phase;
2. no-human operation for that phase where applicable;
3. deterministic replay for the relevant event chain;
4. actor-grounded view-model checks;
5. provenance/debug explanation for the relevant decisions;
6. schema/fixture validation for authored data;
7. regression fixtures proving the forbidden shortcuts are absent.

## First-proof center

The first proof is **The Missing Property Village**.

The proof is narrow by design. It is about ordinary people, physical property, subjective expectation, absence, search, speech/report, institutional record-making, wrong suspicion, actor possession parity, no-human operation, and replayable causal explanation.

The first proof is **not**:

- roads;
- caravans;
- beasts;
- combat;
- adventurer parties;
- companion recruitment;
- bounty boards;
- wilderness hunts;
- LLM chatbots;
- graphical presentation;
- large regional simulation.

Those may be future work only after the first proof survives its gates.

## Execution anti-contamination rule

Committed execution docs must not contain:

- stale target commit IDs;
- session-specific exact-URL fetch ledgers;
- connector failure narratives;
- branch names used as authority;
- claims that a specific review commit is current `main`;
- references to uploaded temporary source material as enduring product authority.

Exact-commit evidence ledgers are required for audits, but they are audit outputs, not execution doctrine.

## Maintenance rule

Correct this folder before implementation if it leaves a coding agent needing to ask any of these questions:

- what counts as scripting;
- how beliefs differ from facts;
- what the first proof is proving;
- how human possession differs from AI control;
- how speech affects beliefs;
- whether LLMs can mutate state;
- whether money/property is physical;
- whether story sifting can intervene;
- how to test no-human operation;
- how to prove replay/provenance.

If those answers are unclear, execution is not ready.
