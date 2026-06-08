# Downstream Staleness and Code Implications Report

## Scope

This report captures downstream staleness and code implications discovered during the 0-foundation reevaluation. It does not rewrite architecture, execution, reference documents, or code. It exists to prevent the implications of the strengthened foundation from being lost.

## Evidence basis

The staleness analysis relies on exact-commit reads of the requested foundation/spec/code files plus supporting reads of selected downstream documents.

The target commit remains:

```text
8ff476f0e1b8a56599c7db05c960329e28768332
```

## Likely stale `docs/1-architecture/**` documents

The architecture layer is closer to the strengthened doctrine than the foundation was. In particular, `docs/1-architecture/14_ACTOR_KNOWN_AUTONOMY_TRANSACTION.md` already states much of the correct transaction boundary. After this foundation pass, however, the architecture layer should be reviewed for conformance to the new foundation doc 14 rather than treating its own doc 14 as a one-off Phase 3A clarification.

Likely affected documents:

- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` — update constitutional spine and universal questions to reference foundation doc 14 explicitly.
- `docs/1-architecture/02_ACTION_AFFORDANCE_SCHEDULING_AND_PROPOSAL_PIPELINE.md` — strengthen proposal/validation split with the exact "truth may validate but not plan" doctrine.
- `docs/1-architecture/03_EVENT_LOG_REPLAY_PROJECTIONS_AND_SAVE_PACKAGES.md` — ensure cognition, decision traces, stuck diagnostics, hidden-truth audits, and provenance graphs are explicit replay obligations.
- `docs/1-architecture/05_CLAIMS_BELIEFS_TRACES_MEMORY_AND_INFORMATION_FLOW.md` — align taxonomy with the new foundation definitions for actor-known truth, hearsay, lie, stale belief, expectation, contradiction, and debug-only omniscience.
- `docs/1-architecture/07_AGENT_COGNITION_ROUTINES_PLANNING_AND_AGENT_LOD.md` — ensure resourcefulness, plan repair, and sealed actor-known context are architectural gates, not aspirations.
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — cross-reference the truth firewall and clarify why-not actor-visible/debug-only layering.
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_METRICS_AND_ACCEPTANCE_ARCHITECTURE.md` — add explicit gates for transaction bypasses and hidden-truth proposal contamination.
- `docs/1-architecture/14_ACTOR_KNOWN_AUTONOMY_TRANSACTION.md` — probably good in substance, but should be reframed as implementing foundation doc 14 rather than being the highest statement of the doctrine.
- `docs/1-architecture/15_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md` — align research notes with the foundation research decisions and remove any mismatch of source authority.

## Likely stale `docs/2-execution/**` documents

Likely affected documents:

- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` — add foundation doc 14 to the authority implications.
- `docs/2-execution/02_PHASE_LADDER_AND_ACCEPTANCE_GATES.md` — add truth-firewall and actor-decision-transaction progression gates.
- `docs/2-execution/06_PHASE_2_EPISTEMICS_VIEW_MODELS_AND_POSSESSION_PARITY.md` — strengthen actor-known context, provenance-bearing cognition, and debug quarantine before Phase 3.
- `docs/2-execution/07_PHASE_3_NEEDS_ROUTINES_AND_NO_HUMAN_LIFE.md` — update Phase 3 no-human gate to require sealed actor-known transaction evidence, not merely plausible day behavior.
- `docs/2-execution/09_DATA_AUTHORING_VALIDATION_AND_GOLDEN_FIXTURES.md` — reject hidden truth in routine assignments, fixture possibility spaces, records, notices, and prehistory.
- `docs/2-execution/10_TESTING_DEBUGGING_OBSERVABILITY_AND_METRICS.md` — add adversarial leakage tests: debug/validation truth is present, actor lacks it, and planner still cannot use it.
- `docs/2-execution/11_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_AND_REGIONAL_EXPANSION.md` — align notices/travel/regional expansion with provenance-bearing summary events and institution-known procedure.

## Likely stale `docs/3-reference/**` documents

Likely affected documents:

- `docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — add session checklist questions for truth firewall, sealed actor-known transaction, and debug quarantine.
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — add risks for "truth validates but also plans," "scheduler cognition authority," "string diagnostic as proof," "routine label dispatch," "empty epistemic projection treated as safe," and "LOD promotion with unexplained knowledge."
- `docs/3-reference/02_GLOSSARY.md` — add or sharpen terms: actor-known cognition transaction, truth firewall, pipeline validation truth, debug omniscience, unproven raw physical truth, provenance class, typed decision trace, stuck diagnostic, institution-known context.

## Code implications discovered

No code changes were made. These are later-session implications.

### Scheduler perception seam

`crates/tracewake-core/src/scheduler.rs` now invokes `ActorDecisionTransaction`, which is directionally correct. However, the scheduler still appears to derive a `VisibleLocalPlanningState` from `PhysicalState` for local food sources, workplace assignments, and sleep places before building actor-known planning state. That may be acceptable only if it is treated as a perception/known-assignment adapter with explicit provenance and visibility limits. Later architecture/code work should verify that no raw physical truth enters planning without provenance.

### Actor-known provenance is promising but should become the hard ingestion boundary

`crates/tracewake-core/src/agent/actor_known.rs` defines actor-known facts and provenance-like distinctions. This is the right seam. Later work should make sure all planner-facing facts are constructed through restricted constructors and audited structurally, not by convention.

### Decision trace proof remains partly string-adjacent

`crates/tracewake-core/src/agent/decision.rs` and scheduler trace output include canonical strings and hidden-truth audit indicators. The target should be typed or structurally inspectable diagnostics that display strings summarize. Tests should avoid treating display text as proof.

### `epistemic_projection: None` needs explicit limitation semantics

The no-human pipeline context may run with no projection and then build planning state with projection limitation. That is acceptable only if the limitation is visible, typed, and prevents broad claims of actor-known completeness. Later work should consider a mandatory `ActorKnowledgeInterface` or `ActorKnownContextBuilder` abstraction.

### Shared pipeline validation should read live authoritative state but leak only modeled feedback

Validators may inspect truth. Later code review should ensure validation failure reasons are split into actor-visible and debug-only layers and that actor-visible feedback becomes cognition only through modeled events/observations/memory updates.

### Routine and need shortcuts should remain guarded

Specs `0006`-`0008` already identified forbidden shortcuts: direct primitive actions from routine labels, direct actions from need thresholds, work forged by proposal need parameters, and `continue_routine` counted as progress. These should remain static/anti-regression checks.

### Content validation needs provenance gates

Fixture/content loaders should reject action-relevant actor knowledge, records, notices, routine assignments, work/home/bed knowledge, and prehistory claims that lack source tags.

### LOD is not implemented here but must not inherit contaminated assumptions

When LOD/regional simulation arrives, it must carry causal and epistemic ancestry in summary events before promoted actors can act on summary knowledge.

## Later-session handling

Recommended sequence:

1. Update architecture docs to implement foundation doc 14.
2. Update execution docs and acceptance gates to require transaction evidence and hidden-truth adversarial tests.
3. Update reference glossary/risk checklist with new terms and shortcuts.
4. Audit code for scheduler/planner ingestion boundaries.
5. Audit tests to ensure they prove typed provenance rather than strings or plausibility.
6. Add content-validation rules for provenance-bearing starting knowledge and routine assignments.

No implementation tickets were produced because this session was a constitutional replacement session, not a code planning session.
