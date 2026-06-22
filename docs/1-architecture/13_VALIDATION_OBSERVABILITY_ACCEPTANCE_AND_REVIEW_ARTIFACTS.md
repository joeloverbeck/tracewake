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
| Observer-only emergence-evidence record | Retrospective review evidence, not world state or holder-known/institution-known context; source run, seed/random provenance where applicable, controller mode, phenomenon family, source event/causal-chain references, extraction time, review/projection version, and enough event-log ancestry to replay and explain the phenomenon |

An observer-only emergence-evidence record is a review artifact over events and
projections. It is not validation input, scheduler input, cognition input,
authoring objective, holder-known context, institution-known context, or world
state. Its path is one-way: authoritative simulation produces events and
projections; an observer/review process may classify phenomenon families such
as contradictions, replans, interruptions, stale-belief consequences,
modeled-channel corrections, belief/truth divergence, wrong suspicion, and
record effects; the resulting review artifact never feeds cognition,
validation, scheduling, candidate generation, LOD promotion, event-spawning
story sifting, or authoring objectives. It must not name required story beats,
dramatic objectives, or numeric floors. Story sifting may contribute to this
observer-only review path only under the incident/lead boundary in
`11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md`; it may not mint
diegetic evidence.

An emergence-evidence artifact is invalid if it is fabricated from debug output
or fixture-only shortcuts instead of path-under-test ancestry, if its review
measurements are used to steer simulation behavior, or if any row is
untraceable to modeled causes.

For TUI-facing simulation work, a feature's declared playable capabilities form
a conformance set. Each member must map to typed causal evidence,
actor-knowledge evidence, a surface disposition, rendered evidence where the
capability is playable, negative evidence where hidden or stale states matter,
and replay or no-human evidence where the capability can affect autonomous or
replayed behavior. The conformance report must be deterministic so reviewers
can distinguish real coverage changes from ordering, rendering, or harness
noise. A missing capability entry, missing witness, or non-playable
classification without architecture rationale is an acceptance failure.

For every validation, anti-contamination, replay, projection, or diagnostic
guarantee, the architecture surface must expose typed, path-under-test
observability: the responsible layer, source event/proposal/context IDs,
checked facts, behavior-witness fields, accepted/rejected stage, and enough
ancestry to distinguish production-path behavior from fixture or harness
fabrication. An artifact's existence, shape, count, checksum, or display text
is insufficient unless paired with typed behavior evidence appropriate to the
protected claim. Architecture defines this observability contract so execution
can attach live negative and mutation/metamorphic checks; execution owns the
procedures and acceptance mechanics.

Temporal observability is part of that typed evidence shape. Decision traces
must identify temporal premises used by candidate generation or method
selection and the provenance for those premises. Validation reports must
separate temporal truth checks from actor-visible temporal reasons. Scheduler
diagnostics must record due effects, deferred or skipped cognition, budget
exhaustion, starvation or fairness symptoms, and responsible layer attribution.
TUI and view-model reports must prove temporal display labels came from
holder-known sources rather than raw clock or debug truth. LOD and replay
artifacts must preserve temporal ancestry and information ancestry. Acceptance
artifacts reject display-string-only proof of temporal correctness.

World-step time-control acceptance requires typed evidence for the composed
path, not only isolated unit behavior. Required proof includes one-tick
human/no-human differential evidence over the same loaded-world progression,
duration-terminal and accounting witnesses for sleep/work completion,
replay-visible evidence for otherwise-empty ticks, actor-known interval-summary
anti-leak evidence, body-exclusive reservation-conflict evidence, and the
standing TUI capability parity extension for time controls and interval
summaries.

Fairness and starvation review artifacts record which holders or processes were
deferred, skipped, summarized, degraded, or blocked; why; for how long or
across what source interval; the responsible layer; and what replay ancestry
supports the outcome. They must include evidence that human-proximity or
possessed-actor priority bias did not influence scheduling, LOD fidelity, or
review classification unless the bias is explicitly non-diegetic input routing
and structurally quarantined from holder knowledge.

Content/schema validators, static guards, manifest checks, and review
artifacts are architecture-protecting boundaries. They reject impossible or
forbidden authoring forms before runtime instead of relying on runtime filters
to clean contaminated data. Their outputs must be structured and
layer-attributed: field, path, or authored element; violated doctrine;
responsible layer; provenance or source status; and author-actionable failure
reason. The validation surface protects against aliases, nested forbidden
concepts, display-string-only proof, hidden-truth cognition fields,
player/human privilege, silent migrations, incompatible content versions, and
outcome chains. Architecture requires this seam and evidence shape; execution
and specs choose schemas, rule languages, commands, compatibility policies, and
error formats.

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
- observer-only review evidence feeds cognition, validation, scheduling,
  authoring, LOD promotion, event-spawning story sifting, or any other
  simulation behavior;
- emergence evidence lacks event-log ancestry to replay and explain the
  reviewed phenomenon;
- an acceptance artifact exists without typed, path-under-test behavior
  evidence for the protected claim;
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
