# Validation, Observability, Acceptance, and Review Artifacts

## Status

Authoritative architecture contract.

## Purpose / core rule

Typed diagnostics are architecture, not polish. Acceptance must prove causality, replay, holder-known filtering, ordinary proposal paths, TUI embodied play, debug quarantine, and anti-contamination. Plausible behavior without proof fails.

## Authority owned

This document owns architecture-level acceptance artifacts, observability obligations, anti-regression gates, review checklists, and definitions of invalid pass conditions.

## Authority denied

Acceptance may not rely on display strings, manual inspection alone, “looks right,” hidden debug state, branch-specific behavior, or LLM outputs. It may not waive TUI or no-human gates because the kernel tests pass.

## Contract

### Required artifact families

| Artifact | Must prove |
|---|---|
| Event log | Causal, ordered, schema-versioned events with causes, streams, participants, and replay material |
| Replay report | Live vs rebuilt projections, checksums, unsupported event/version failures, and deterministic outcomes |
| Holder-known context packet | Sealed context, holder, trigger, provenance list, explicit unknowns, context hash, forbidden-truth audit |
| Decision trace | Candidates, selected/rejected goals, intention lifecycle, method selection, local plan, proposal ancestry, hidden-truth audit |
| Stuck diagnostic | Typed blocker category, actor-known explanation, attempted action/method, fallback/retry outcome, debug-only details |
| Validation report | Stage, reason codes, checked facts, actor-visible summary, debug summary, event IDs, mutation status |
| Epistemic projection report | Observations, beliefs, contradictions, notebooks, holder filtering, source links, checksums |
| Institution procedure trace | Institution-known context, records/reports/roles/resources, procedure candidates, selected method, proof threshold, failure/sanction evidence |
| TUI transcript/view-model evidence | Bound actor, semantic action IDs, embodied output, actor-legible why-not, notebook, debug separation |
| Debug report | Non-diegetic truth/belief/procedure comparison that cannot feed embodied surfaces |
| Content validation report | Source tags, schema versions, forbidden content patterns, fixture provenance, no outcome scripts |
| Anti-regression/static guard | Detection of scheduler shortcuts, string-as-proof, player privilege, debug leakage, quest ontology, LLM authority |

### Acceptance gate groups

#### No-human ordinary-life gate

Must show unpossessed actors acting through ordinary decision transactions and shared pipeline events. Required evidence includes actor roster, decision order, time windows, ordinary action events, waits/failures, no controller/player-conditioned events, decision/stuck traces, and replay match.

#### Actor-knowledge filtering gate

Must include hidden truth present in authoritative state but absent from actor-known context. The actor must not plan with it, TUI must not show it, and why-not must not reveal it. Debug may show the comparison only as non-diegetic.

#### Proposal/validation gate

Every world-affecting action from human, scheduler, institution, or future language parser must produce an ordinary proposal, validation report, and event/rejection artifact.

#### Replay/projection gate

Live state and replay-rebuilt projections must match for physical, agent, epistemic, institution, diagnostic, and metrics projections relevant to the test. Unsupported versions must fail loudly.

#### TUI embodied gate

TUI must bind actor, render actor-filtered view, expose stable semantic action IDs, submit ordinary proposals, rerender after events, show actor-legible why-not, show notebook, run no-human controls, and keep debug separated.

#### Institution gate

Institution procedures must use institution-known context from records/reports/roles/resources/procedure state. They must separate violation, detection, suspicion, proof, record, sanction, and appeal/failure.

#### Content gate

Content fixtures and domain packs must validate source/provenance tags for actor knowledge, records, notices, household facts, work/home/bed knowledge, routines, institutions, and prehistory. Forbidden fields include culprit, quest step, next clue, guaranteed reward, true target for planner, and hidden director trigger.

#### LLM-disabled gate

All acceptance-critical behavior must run with LLMs disabled. Deterministic templates/mocks are allowed for language surfaces.

### Invalid pass conditions

A test does not pass if:

- an actor reaches true food/work/sleep route without known-context provenance;
- a scheduler emits primitive actions from needs or routine labels;
- a workplace table chooses a plan;
- an institution knows event-log truth without modeled report/record/procedure;
- a TUI why-not reveals validation/debug truth;
- a debug panel shares embodied view data without quarantine;
- a story/projection spawns events or drives decisions;
- a display string is the only proof;
- replay matches final state but decision ancestry is missing;
- an LLM output is required to make the test pass.

### Review checklist

Before accepting a subsystem, reviewers must ask:

1. What is the holder-known context?
2. What are the provenance sources?
3. What hidden truth was available to validators/debug but unavailable to cognition?
4. What proposal entered the shared pipeline?
5. What events or rejections were committed?
6. What feedback became future knowledge, if any?
7. What typed diagnostics prove the decision/failure?
8. What TUI embodied/debug views exercise it?
9. What replay proof exists?
10. What anti-pattern test would catch the likely shortcut?

## Required diagnostics / replay / TUI hooks

This document is itself the diagnostic checklist. Each subsystem document must name the artifacts it contributes. The architecture is incomplete if a feature has no acceptance artifact strategy.

## Acceptance implications

The canonical first proof is missing-property/no-human ordinary life. Later systems can be designed but not accepted as proof substitutes. Acceptance must be adversarial: include tempting hidden truth, tempting debug leakage, tempting routine shortcuts, and tempting player privilege.

## Anti-patterns

- “We inspected the code and it seems fine.”
- “The rendered text says actor-known.”
- “The no-human run ended with plausible state.”
- “The debug view is behind a keybinding, so leakage cannot matter.”
- “The LLM can explain why it acted.”
- “Fixture validity is guaranteed by the author.”

## Cross-document obligations

This document validates every other architecture document. Any new architecture feature must update the artifact list or explicitly state why existing artifacts cover it.
