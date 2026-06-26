# Reference Index and Review Checklist

## Status

This is the compact operating index for Tracewake's reference layer. It exists for future AI coding, specification, documentation, fixture, schema, prompt, and review sessions that need fast guardrails without rereading the full foundation, architecture, and execution layers.

The reference layer is now realigned to the post-overhaul foundation, architecture, and execution spine. It is not a constitution, architecture contract, execution plan, roadmap, source-note bibliography, issue tracker, or implementation-certification artifact. It must stay small and low-duplication.

When this file conflicts with the foundation, architecture, or execution layers, use the higher-authority layer and update this file only if the drift is recurring enough to justify reference-layer maintenance.

## Authority boundary

This file owns only:

- the reference-layer map;
- the exact-source discipline prompt for repository-analysis sessions;
- compact review questions for coding, spec, fixture, schema, prompt, and acceptance-review work;
- deferred-term reminders;
- phase-exit and gate-exit review prompts.

It does not define doctrine, gate semantics, subsystem architecture, implementation plans, tickets, or proof criteria. Gate codes are named here only as lookup anchors; their definitions live in `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` and `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`.

## Depends on

- `docs/0-foundation/00_FOUNDATION_INDEX.md`
- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`

## Reference layer contents

- `00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md` — session-start source discipline, compact review checklist, deferred-term handling, and phase/gate-exit review prompts.
- `01_DESIGN_RISK_REGISTER.md` — operational risk register for recurring failure modes that can look locally correct while undermining Tracewake.
- `02_GLOSSARY.md` — prescriptive terminology control across truth, observation, belief, memory, claim, speech act, trace, record, institutional fact, projection, debug truth, holder-known context, source discipline, and gate vocabulary references.

Do not merge the risk register and glossary. They do different jobs: the risk register watches failure modes; the glossary controls names and boundaries. Do not add reference files merely for symmetry. Add one only when a compact lookup or review aid is repeatedly needed across foundation, architecture, execution, implementation, fixtures, schemas, tests, and AI-session handoffs.

## Authority order

For design meaning, use this order:

1. Exact fetched source files from the target repository and target commit.
2. Foundation layer doctrine.
3. Architecture layer contracts and subsystem boundaries.
4. Execution layer gate vocabulary, certification sequence, and first-proof constraints.
5. Reference layer guardrails.
6. Current AI-session inference.
7. Prior chat, memory, repository labels, branch names, connector names, filenames, or unchecked local context.

The lower levels may summarize or operationalize the higher levels, but they do not override them. A reference-layer entry is a review aid, not permission to bypass a foundation invariant, architecture boundary, or execution gate.

## Source discipline gate for AI sessions

Before analyzing repository content or producing replacement docs, implementation guidance, tests, schemas, tickets, or review findings, a session must pass this gate.

1. Treat the user-supplied target repository, target commit, and uploaded manifest as the source request.
2. Treat the manifest only as a path inventory. It does not prove branch state, latest status, file freshness, repository identity, or semantic authority.
3. Fetch each needed file by exact URL containing repository owner, repository name, full target commit, and the manifest path.
4. Do not clone, use code search, use repository-scoped connector arguments, ask for the default branch, fetch by branch name, infer from tool namespace labels, or rely on snippets.
5. Before using each fetched file, verify that the requested URL contains the exact owner, repository, commit, and manifest path.
6. Record an evidence ledger before analysis. It must state repository, commit, freshness claim, manifest role, URL fetch method, fetched exact URLs, whether clone/search/branch/default metadata were used, whether contamination was observed, and whether connector/tool namespace labels were trusted.
7. Abort before analysis if a needed file cannot be fetched from an exact target-commit URL, if a requested path is absent from the uploaded manifest, if a fetch rewrites to another repository or branch, if any fetched source points to another repository, or if the analysis would depend on unfetched files, snippets, memory, or prior chat.
8. Treat stale statements inside a correctly fetched file as document content, not repository provenance. The exact fetch URL and evidence ledger decide provenance.

If a fetched document names another commit, branch, old repository, obsolete scope, or prior session, flag stale-document drift and continue only if the source URL itself remains exact and uncontaminated.

## Gate-code lookup posture

When a review touches certification, name the relevant execution gate codes but do not redefine them. Definition sources are:

- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` for `P0-DOC`, `P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, and `DIAG`.
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` for `SPINE-CERT`, `EPI-CERT`, `ORD-LIFE-CERT`, `FIRST-PROOF-CERT`, `PHASE-4-ENTRY`, and `SECOND-PROOF-ENTRY`.

Use archived specs `0005` through `0008` only as history unless a live execution document explicitly promotes the idea under current doctrine. Landed historical work is not post-overhaul certification. A claim that Phase 3A is certified must point to the appropriate live gate artifact, especially `P0-CERT`, `EPI-CERT`, `ORD-LIFE-CERT`, and `FIRST-PROOF-CERT` where relevant.

## Compact doctrine review checklist

Reject or escalate a proposal, schema, fixture, test, prompt, UI surface, or code plan if any answer is unsafe.

### Source and provenance

- Was every repository fact fetched from the exact target commit and listed in the evidence ledger?
- Is any claim imported from branch state, repository metadata, code search, connector labels, old chats, old filenames, hidden scratchpad, or memory?
- Can each design assertion be traced to fetched source doctrine, deliberate inference, or explicitly named external research?
- Are stale source strings, old commit mentions, or incompatible prior-session assumptions being treated as authority?
- Are archived specs `0005` through `0008` used only as historical evidence unless live execution docs promote the concept?

### Authority and mutation

- Does every world-affecting ordinary-play change pass through command/proposal, validation, scheduling or resolution, event commit, projection rebuild, replay, and diagnostics as required by the live `PIPE`, `NO-DIRECT`, `REPLAY`, and `DIAG` gates?
- Is any UI, prompt, LLM output, prose summary, debug view, story sifter, fixture script, or presentation layer creating authoritative simulation fact?
- Are debug injections explicit test/debug metadata rather than ordinary play?
- Are validation reports and why-not explanations generated from the shared pipeline rather than reimplemented in a client?
- Does a claimed certification name the live gate artifact instead of saying a historical phase “landed”?

### Holder-known and epistemics

- Who is the holder at each cognition/procedure/view boundary: actor, institution, household, role office, speaker, listener, embodied viewer, TUI affordance selector, lead interpreter, LOD promotion recipient, regional procedure owner, or other process named by architecture?
- Was a sealed holder-known context built before candidate generation, method selection, local planning, speech interpretation, institutional procedure choice, embodied affordance selection, lead interpretation, or LOD promotion?
- Does every context input have provenance, uncertainty/staleness handling, and explicit unknowns where hidden truth would be tempting?
- Are ground truth, observation, holder belief, memory, claim, speech act, trace, record, institutional fact, projection, debug truth, and player-facing surface wording kept distinct?
- Does embodied mode expose only actor-known information, uncertainty, and source-backed inferences?
- Can a false belief, stale record, wrong suspicion, or unsupported claim persist without being silently corrected from truth?
- Does possession change control and viewpoint only, not identity, memory, guilt, privilege, or knowledge?
- Is `TFW` satisfied by reference to live execution evidence rather than plausible behavior?

### Temporal authority

Use this block as a pointer to foundation `03` / `INV-112`, the architecture temporal and holder-known contracts, and the execution truth-firewall / scheduler / view-model proof homes. It is not new doctrine.

- Does every planning, procedure, embodied-view, speech, lead, or LOD temporal premise have a holder-known or institution-known source rather than raw validator, replay, or scheduler time?
- Is validator, replay, and scheduler time separated from diegetic cognition, procedure conclusions, embodied surface wording, speech interpretation, lead urgency, and institution-known procedural time?
- Are time acceleration and debug temporal panels quarantined from embodied surfaces and ordinary speech, records, leads, and procedure state?
- When summaries, snapshots, projections, compaction, promotion, or demotion carry temporal claims forward, is temporal ancestry preserved with enough information ancestry to explain who could know what?
- Do human time controls advance the same world step as no-human and autonomous progression, rather than a private possessed-actor clock or TUI-owned simulation path?
- Does every `(actor, need, tick)` have one accounting classification, including human waits, passive deltas, sleep/work duration effects, and no-human progression?
- Do open durations close through the shared authority for event-sourced starts and terminals, not through batch-local pending queues or client-side completion logic?
- Is interval output positive actor-known evidence from modeled sources, not a redacted omniscient event diff or an omniscient "nothing happened" summary?

Current 0052 evidence answers these prompts by constructing the runtime through
`LoadedWorldRuntime::from_bootstrap` from a validated `LoadedWorldBootstrap` and
routing human-origin and no-human-origin advancement through core-derived loaded
actors, declared processes, closed actor transaction outcomes, fail-closed event
identity, temporal replay verdicts, and a core-owned interval product.
Reviewers should look for `world_step_coordinator.rs`, `generative_lock.rs`,
`replay_temporal_frontier.rs`, `holder_known_interval_projection.rs`,
`salient_stop_actor_known.rs`, `negative_fixture_runner.rs`,
`command_loop_session.rs`, `playable_capability_parity.rs`, `embodied_flow.rs`,
`parity_adversarial.rs`, `archive/tickets/0052FOUCONFOU-009.md`,
`archive/tickets/0052FOUCONFOU-010.md`, and
`archive/tickets/0052FOUCONFOU-011.md` before treating a
loaded-world/time-control row as current executable evidence. The required
checks to confirm operationally are `public-boundary conformance` and `mutation
shard reconciliation (lock layer)`.

### Ordinary-life play

- Can the scenario run without a human controller and still produce ordinary life, needs, routines, interruptions, and consequences under `NO-HUMAN`?
- Does the currently possessed actor remain ordinary, with no privileged action, knowledge, scheduling, plot gravity, or protected role under `POS-PARITY`?
- Is the feature reachable and inspectable through the TUI or through stable view-model tests before it is considered accepted?
- For every capability added or changed, where is its declared surface disposition and real-pipeline actor-filtered witness?
- Are no-human runs, possession parity, holder-known view filtering, replay, fixture coverage, and debug explanation part of acceptance through the live execution gates?

### Institutions and social machinery

- Do institutions act from institution-known context: reports, records, roles, procedures, resources, jurisdiction, norms, evidence thresholds, bias, and institutional memory rather than truth?
- Are violation, detection, suspicion, report, proof, sanction, notice, and record lifecycle separate?
- Does every record have provenance, artifact or storage context, author or issuer, claims, readers, amendments, and lifecycle events where applicable?
- Is any global legal meter, automatic proof, truth-reading notice, or record without reporter/issuer being smuggled in?
- Is Phase 4 institution/record/wrong-suspicion work blocked until `PHASE-4-ENTRY` is satisfied under execution doctrine?

### Language, story, and presentation

- Are LLMs outside simulation authority and disabled or mocked for acceptance-critical operation?
- Are speech acts structured propositions with speaker stance, source, listeners, and validation?
- Is listener interpretation based on listener-known context rather than ground truth or authorial intent?
- Is story sifting retrospective and non-mutating, with salience unable to pace, spawn, select, or repair events?
- Does player-facing wording remain a surface over canonical state rather than ontology?

### Content, schemas, and fixtures

- Are content packages defining possibility space, not hidden outcomes?
- Do fixtures seed initial state and test chains without forcing ordinary-play result sequences?
- Are schema names aligned with the glossary, with stable IDs, versioning, provenance, source-backed beliefs, content-version compatibility, and rejection of forbidden core terms?
- Do random draws have replayable seeds or recorded draw labels sufficient for audit?
- Do fixture families include adversarial hidden-truth and contamination cases, not only friendly golden paths, consistent with `FIXTURE`?

### Scale and deferral

- Is the first playable proof still small, local, TUI-first, replayable, inspectable, no-human-capable, and centered on missing expected property, belief divergence, records, and wrong suspicion?
- Is Phase 4 blocked by `P0-CERT`, `SPINE-CERT`, `EPI-CERT`, `ORD-LIFE-CERT`, `FIRST-PROOF-CERT`, and `PHASE-4-ENTRY` as applicable?
- Are notices as central play, broad travel, route threats, bounties, companions, proof/payment flows, regional LOD, story-sifting projections, and broader world history blocked until `SECOND-PROOF-ENTRY` or another live execution entry rule permits them?
- If LOD or compaction is introduced, are active traces, beliefs, records, leads, suspicions, procedures, and causal ancestry preserved?

## Deferred term handling

Deferred terms are not banned forever. They are unsafe when they harden before the proof that can support them.

- **Notice** is a canonical artifact, but notice-board-centric play belongs after the first proof unless used as a small local artifact with explicit author, claims, readers, lifecycle, and stale risk.
- **Bounty** is allowed later as a public contract, obligation, report, sanction, or payment procedure. It is not an objective menu item.
- **Companion** is allowed later as an autonomous actor, recruited actor, co-traveler, escort, contract party, or follower with needs, beliefs, refusal, and independent action. It is not a privileged party slot.
- **Travel** and **route threat** are allowed later as spatial actions and boundary processes with observation, evidence, uncertainty, route knowledge, resources, and consequences. They are not hidden pacing tools.
- **Proof** and **payment** are allowed as institutional or contractual procedure outcomes, not as completion flags or guaranteed rewards.
- **Regional history**, **LOD**, and broad **story sifting** are allowed only with explicit ancestry preservation and no human-proximity probability bias.

A deferred term entering first-proof core must justify itself as a small local artifact or process required by the first proof. Otherwise, move it back behind the appropriate execution entry gate.

## Phase-exit / gate-exit review prompt

At every phase exit, gate exit, certification review, or expansion handoff, answer these questions in writing or in the review artifact:

1. Which live gate codes are being claimed, and where are their definition sources and pass artifacts?
2. What would still work if the human controller never possessed anyone?
3. What does each relevant holder know, believe, report, remember, suspect, infer, record, select, or interpret, and from what source?
4. What sealed holder-known context and provenance packet fed each cognition/procedure/view decision?
5. What event, validation report, random draw, trace, record, projection rebuild, context packet, decision trace, or debug query proves the causal chain?
6. Which hidden truth was available to validators/debug but excluded from cognition, procedure selection, affordance generation, embodied TUI, institutions, leads, and LOD promotion?
7. Which risk-register entries are active, newly escalated, or ready for retirement, and what evidence supports that status?
8. Are archived specs or tickets being cited only as history, not as certification under post-overhaul doctrine?

A phase or gate cannot exit on demo success alone. It needs replay evidence, no-human evidence, holder-known view/procedure evidence, negative/failure evidence, TUI or view-model evidence, fixture evidence, diagnostics, and causal explanation evidence appropriate to the live execution gate being claimed.

## Maintenance rule

When a future session proposes reference-layer edits, keep only material that future AI sessions need at the moment of coding, spec writing, fixture authoring, schema review, prompt construction, or phase/gate acceptance. Move doctrine back to foundation, contracts back to architecture, gate semantics and proof order back to execution, research bibliography back to source-note files, and implementation tasks out of the reference layer entirely.
