# Design Risk Register

## Status

This reference document is Tracewake's operational watchlist for unresolved, recurring, or easily regressing design risks. It is for future AI coding/spec sessions, code review, data authoring review, test review, and phase/gate-exit review.

The register is now realigned to the post-overhaul foundation, architecture, and execution spine. It is not a constitution, architecture contract, roadmap, issue tracker, gate-definition source, or list of accepted tradeoffs. Settled doctrine belongs in the foundation and architecture layers. Gate semantics and certification order belong in the execution layer.

This register exists because Tracewake has relapse modes that can appear locally reasonable while destroying the product premise. The risk register and glossary must remain separate. The register tracks failure modes. The glossary controls terms.

## Authority boundary

This file owns only compact risk memory: recurring symptoms, guardrails, evidence to watch, escalation triggers, retirement criteria, and review cadence. It may name execution gate codes as cross-references, but it does not define those gates or certify that code has passed them.

Gate definitions live in:

- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` for `P0-DOC`, `P0-CERT`, `TFW`, `PIPE`, `NO-DIRECT`, `NO-HUMAN`, `POS-PARITY`, `REPLAY`, `FIXTURE`, and `DIAG`.
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md` for `SPINE-CERT`, `EPI-CERT`, `ORD-LIFE-CERT`, `FIRST-PROOF-CERT`, `PHASE-4-ENTRY`, and `SECOND-PROOF-ENTRY`.

## Depends on

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- `docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md`
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`

## How to use this register

Review this register:

- at the start of any AI session that relies on repository content;
- at every phase exit, gate exit, certification claim, and expansion handoff;
- before accepting a new schema, fixture, domain pack, event family, projection, TUI surface, debug command, or language surface;
- before enabling any live LLM surface beyond deterministic templates or deterministic mocks;
- before scaling population, geography, history, regional processes, or LOD;
- after any regression in validation, replay, holder-known filtering, possession parity, TUI playability, debug quarantine, source discipline, or diagnostics.

Status values:

- **Watch** — expected risk; monitor with tests, reviews, metrics, and source discipline.
- **Active** — symptoms are present in docs, fixtures, code, schemas, UI, tests, prompts, or demos.
- **Escalated** — blocks analysis, implementation acceptance, certification, or phase/gate exit.
- **Dormant** — no symptoms in the current phase, but likely to recur under future pressure.
- **Retired** — removed only after sustained regression coverage and no plausible near-term recurrence.

A risk can be retired only when the team can name the tests, fixtures, reviews, metrics, or architectural boundaries that would catch its return. Source-discipline, contamination, and terminology risks should be treated as permanent watch risks for AI-assisted work.

## Risk watchlist

### R-00 — Exact-Commit Drift

- **Why it matters:** Future sessions can silently analyze the wrong Tracewake by using branch state, search snippets, repository-scoped connector calls, old filenames, or memory instead of the requested commit.
- **Symptoms:** Fetches from `main`, `master`, `HEAD`, a branch name, code search, repository metadata, connector namespace labels, local clone state, or URLs missing the requested owner, repository, commit, or path.
- **Mitigation / guardrails:** Fetch every needed repository file by full exact raw URL. Treat the manifest as path inventory only. Record an evidence ledger before analysis. Abort if a needed file cannot be fetched from the exact target commit.
- **Evidence to watch:** URL list; fetch method; visible citations or source paths; any branch/default-branch lookup; any result whose source URL omits the target commit.
- **Escalation trigger:** Any ordinary analysis, generated replacement file, implementation recommendation, or review finding depends on a branch fetch, search result, snippet, unfetched file, memory, or connector label.
- **Retirement criteria:** Never retire for AI-assisted repository work. Exact-commit discipline is a standing gate.
- **Status / review cadence:** Watch. Review at every session start before analysis.

### R-01 — Repository Contamination

- **Why it matters:** Evidence from another repository can look semantically plausible while invalidating every downstream correction.
- **Symptoms:** A fetched URL, citation, tool result, file source, repository label, or visible content points to another repository; stale paths name a prior project; a connector silently rewrites a request; a generated artifact preserves the wrong repository identity.
- **Mitigation / guardrails:** Trust only exact URLs containing `joeloverbeck`, `tracewake`, the target commit, and the manifest path. Do not trust connector namespace labels or prior chats. Abort on any fetched source from another repository or branch.
- **Evidence to watch:** Raw source URLs, redirect targets, repository owner/name in every fetch, generated evidence ledgers, and any mention of an unrelated repository in source provenance.
- **Escalation trigger:** Any fetched result or cited source points to a repository other than `joeloverbeck/tracewake`, or any generated doc treats another repository as evidence.
- **Retirement criteria:** Never retire. Repository identity is a standing source-authenticity gate.
- **Status / review cadence:** Watch. Review at every source fetch and artifact handoff.

### R-02 — Provenance Collapse

- **Why it matters:** Tracewake's design depends on knowing where information came from. AI sessions fail the same way simulated holders fail if fetched source, inference, external research, old memory, and generated wording blur together.
- **Symptoms:** Claims have no source; a doc says “the repository does X” without fetched evidence; external research becomes product doctrine without a boundary; generated replacement text does not distinguish exact source, compact reference synthesis, deliberate inference, and invented convenience.
- **Mitigation / guardrails:** Keep a source ledger. Distinguish fetched source doctrine, compact reference synthesis, deliberate inference, and external research. Prefer guardrail wording over claims of implementation reality unless source evidence proves implementation reality.
- **Evidence to watch:** Review notes, replacement docs, prompt packets, issue comments, tests, and certification artifacts that cannot answer “where did this assertion come from?”
- **Escalation trigger:** A phase decision, gate claim, schema, or implementation review depends on unproven provenance or silently imported context.
- **Retirement criteria:** Retire only for a bounded artifact class with automated provenance checks and review enforcement; otherwise keep active as an AI-session risk.
- **Status / review cadence:** Watch. Review at every AI handoff and source-backed design review.

### R-03 — Stale-Session / AI-Handoff Contamination

- **Why it matters:** Long-running or repeated AI sessions can carry stale assumptions from older commits, older drafts, old repository names, hidden scratchpads, or previous answers.
- **Symptoms:** The assistant says “as before” without refetching; output preserves old phase scope; generated docs inherit old commit IDs; references depend on uploaded filenames rather than manifest paths; hidden memory overrides exact-source evidence.
- **Mitigation / guardrails:** Restart every substantive repository analysis from the evidence ledger. Treat prior chat and memory as untrusted. Require exact target commit, fetched file list, and contamination statement before analysis. Reconcile internal document dates and commit mentions against the exact fetch URL.
- **Evidence to watch:** Handoff summaries, AI scratchpads, old target commits in generated files, repeated wording from previous sessions, and claims not traceable to fetched source.
- **Escalation trigger:** A session continues after noticing stale handoff context without naming and quarantining it.
- **Retirement criteria:** Never fully retire for AI-assisted work. It may be dormant within a single tightly scoped, freshly fetched session.
- **Status / review cadence:** Watch. Review at every handoff and every resumed task.

### R-04 — Stale Manifest Trust

- **Why it matters:** A manifest proves only that a path was listed in an uploaded inventory. It does not prove latest branch state, file freshness, semantic authority, or completeness beyond the requested scope.
- **Symptoms:** Treating manifest entries as file contents; fetching paths not listed in the manifest because they “probably exist”; assuming a missing path was intentionally deleted; using the manifest to claim current `main`; ignoring needed files because they were not convenient to fetch.
- **Mitigation / guardrails:** Use the manifest only to identify allowed paths. Fetch all required paths from exact target-commit URLs. Abort if a requested path is absent or a required file cannot be fetched. Do not infer branch state from the manifest.
- **Evidence to watch:** Path inventory, fetched-file list, absent-path handling, generated claims about latest branch state.
- **Escalation trigger:** Any analysis depends on a path not present in the uploaded manifest or on manifest metadata as repository truth.
- **Retirement criteria:** Never retire when user-supplied manifests are part of the workflow.
- **Status / review cadence:** Watch. Review at every repository-analysis session start.

### R-05 — Stale-Document Metadata Drift

- **Why it matters:** A file fetched from the correct commit may contain old commit IDs, old dates, outdated scope statements, copied headers, stale source notes, or prior-session assumptions. Those strings can poison future work if treated as authority.
- **Symptoms:** A fetched document names a different target commit; phase labels disagree across layers; source notes imply old proof scope; generated replacement files copy stale metadata; review comments cite an internal old commit instead of the exact fetch URL.
- **Mitigation / guardrails:** Let the exact URL establish repository provenance. Treat stale internal metadata as a risk signal and content issue, not as source authority. Preserve durable doctrine; do not preserve obsolete commit strings unless the replacement explicitly needs a historical note.
- **Evidence to watch:** Commit IDs, dates, branch claims, repository names, path names, and “current” claims inside fetched prose.
- **Escalation trigger:** Generated docs, schemas, tests, or issue comments inherit stale internal metadata as if it were current provenance.
- **Retirement criteria:** Retire only for a specific stale string after replacement and review; the general risk remains permanent.
- **Status / review cadence:** Active when stale metadata is observed; otherwise Watch. Review during reference replacement and source-note updates.

### R-06 — Protagonist Gravity

- **Why it matters:** Tracewake fails if the world organizes around the human controller. The possessed actor must remain ordinary.
- **Symptoms:** Events cluster near the possessed actor without modeled cause; actors wait for human acceptance; opportunities target the human; scheduler pauses unpossessed actors; summaries imply the possessed actor is the intended solver.
- **Mitigation / guardrails:** Run no-human simulations from the beginning. Compare embodied and no-human event distributions. Ban human-conditioned world logic in ordinary systems. Keep possession as controller metadata. Require ordinary-actor parity for world-affecting actions.
- **Evidence to watch:** Human-conditioned event rates, action definitions unavailable to unpossessed actors, fixture fields naming the human controller, idle loops that depend on possession, tests that pass only with a controller bound.
- **Escalation trigger:** Any ordinary-play system checks possession or human-controller identity to decide world events, knowledge, opportunity, scheduling, success, or protection.
- **Retirement criteria:** Sustained `NO-HUMAN`, `POS-PARITY`, scheduler, TUI, and fixture validation coverage.
- **Status / review cadence:** Watch. Review at every runnable phase/gate exit and whenever TUI affordances or scheduler behavior change.

### R-07 — Quest Relapse

- **Why it matters:** Quest ontology imports objective truth, intended human involvement, completion flags, guaranteed rewards, waiting actors, and authored solution paths.
- **Symptoms:** Core types, schema fields, UI labels, tests, or fixtures named `Quest`, `Objective`, `QuestTarget`, `Completion`, `Reward`, `QuestGiver`, or equivalent; notice boards behave like task menus; leads point to hidden truth; actors wait for acceptance.
- **Mitigation / guardrails:** Model incidents, requests, contracts, obligations, records, notices, rumors, leads, suspicions, procedures, proof, payment, and custody transfer instead. Treat leads as holder-known projections with source, uncertainty, and staleness.
- **Evidence to watch:** Objective markers, completion flags, reward spawning, accepted-task conditions, tests asserting solved state instead of world state, belief state, record state, obligation state, and institutional state.
- **Escalation trigger:** Quest-like schema, fixture, planner state, TUI action, or acceptance test enters core ontology.
- **Retirement criteria:** Stable glossary enforcement, schema validation rejecting forbidden core terms, and fixtures proving non-quest alternatives.
- **Status / review cadence:** Watch. Escalate immediately if quest-like ontology enters core.

### R-08 — Action-Pipeline Bypass

- **Why it matters:** The shared action pipeline is the enforcement point for parity, causality, knowledge, norms, traces, scheduling, replay, event commit, and diagnostics.
- **Symptoms:** UI mutates state; fixture scripts move items during ordinary play; domain packs write events directly; planners update components; language surfaces commit facts; storage adapters become domain models; debug commands look identical to ordinary actions.
- **Mitigation / guardrails:** Route every world-affecting ordinary-play change through command or action proposal, validation report, scheduling or resolution, event commit, projection rebuild, replay, and debug explanation. Keep debug/test metadata explicitly marked and non-diegetic.
- **Evidence to watch:** Direct component writes outside event application, state changes without validation reports, authored timed outcome chains, boundary code owning domain rules, tests that inspect current state without event causality.
- **Escalation trigger:** Any ordinary-play mutation bypasses validation and event commit, or claims `PIPE` while skipping shared proposal/validation/replay evidence.
- **Retirement criteria:** Static boundaries, review checks, and tests that fail on direct mutation outside event application.
- **Status / review cadence:** Watch. Escalate immediately on ordinary-play bypass.

### R-09 — Epistemic Leakage

- **Why it matters:** Play depends on divergence between truth, holder-known context, belief, observation, memory, claim, speech, trace, record, institutional fact, projection, debug truth, and surface wording.
- **Symptoms:** Embodied mode shows hidden culprit, true item location, lie labels, private beliefs, or debug summaries; institutions read event-log truth; speech reveals unsupported facts; actor notebooks inherit knowledge from prior possession; affordance menus are generated from raw truth and then visually filtered.
- **Mitigation / guardrails:** Require sealed holder-known contexts on planner, institution, language, lead, LOD, and view-model queries. Use actor-known projections for embodied UI. Test multi-possession scenarios, false beliefs, stale records, contradiction, rumor, wrong suspicion, and adversarial hidden truth.
- **Evidence to watch:** View models built from raw state then manually filtered, tests checking truth instead of holder/source/confidence, hidden event references in prompt packets, actor notebooks sourced from debug projections.
- **Escalation trigger:** Hidden truth becomes visible, actionable, or inferable in embodied mode without modeled observation, memory, record read, speech act, trace, or other permitted provenance.
- **Retirement criteria:** Durable leakage tests, holder-known query APIs, possession-parity fixtures, debug/embodied projection separation, and `TFW` certification evidence.
- **Status / review cadence:** Watch. Escalate on any hidden truth in embodied mode.

### R-10 — LLM, Prose, and Presentation Authority Breach

- **Why it matters:** No simulation fact may be born from prose. LLM output, UI text, cutscene-like wording, summaries, and templates are nondeterministic or presentational unless bound to validated structured state.
- **Symptoms:** Dialogue creates facts; generated text changes state; prompt packets include hidden truth; live model output is required for tests; summaries erase uncertainty; a UI label declares proof, guilt, completion, or reward without institutional state.
- **Mitigation / guardrails:** Use structured speech acts, deterministic templates, deterministic mocks, actor-filtered prompt packets, output validation, unsupported-claim rejection, and LLM-disabled operation as the baseline. Presentation may render or parse proposals; it may not author authority.
- **Evidence to watch:** Test flakiness tied to live generation, generated records without speech/report events, prompt packets containing private truth, acceptance tests golden-locking LLM wording, UI text consumed as rules.
- **Escalation trigger:** LLM/prose/UI output is used as ground truth, holder knowledge, institutional judgment, planner state, event source, proof, or record authority.
- **Retirement criteria:** Live LLM features are behind validation boundaries, deterministic acceptance paths exist, and prompt/output tests prove no authority leakage.
- **Status / review cadence:** Watch now; Active when any live language surface is prototyped.

### R-11 — Debug Truth Leakage

- **Why it matters:** Tracewake needs omniscient debugging, but debug truth must never become holder knowledge, embodied UI fact, ordinary action precondition, institutional knowledge, or acceptance evidence masquerading as ordinary play.
- **Symptoms:** Debug and embodied views share untagged projections; debug notes satisfy action preconditions; actor notebooks include hidden truth; possession history becomes world fact; debug injections are used in ordinary play; forensic summaries appear as actor-known summaries.
- **Mitigation / guardrails:** Keep debug view models visibly non-diegetic. Separate human/debug notes from actor notebooks. Route debug injections through explicit debug/test events. Test that embodied views, agents, language packets, institutions, and fixtures cannot consume debug-only projections.
- **Evidence to watch:** Shared projection types without mode tags, debug fields in embodied prompt packets, notebooks sourced from causal graph truth, tests requiring debug commands to complete normal flows.
- **Escalation trigger:** Debug truth affects ordinary play, actor belief, institution procedure, TUI affordance availability, ordinary validation, or certification evidence for embodied knowledge.
- **Retirement criteria:** Separate projection APIs, debug-mode tests, and negative tests proving debug facts cannot be consumed by embodied systems.
- **Status / review cadence:** Watch. Escalate on any debug truth in embodied mode.

### R-12 — Institutional Omniscience

- **Why it matters:** Institutions are fallible social machines. If guards, clerks, ledgers, notices, or procedures know truth automatically, wrong suspicion, stale records, and social consequence collapse.
- **Symptoms:** Guards know violations instantly; records appear because a violation occurred; proof thresholds read true culprit; sanctions occur without detection or procedure; notices update from truth; suspicion formulas use actual culprit.
- **Mitigation / guardrails:** Separate violation, detection, suspicion, report, record, proof, sanction, notice, closure, and appeal/failure. Require institution-known context: reports, records, roles, resources, jurisdiction, bias, evidence thresholds, public artifacts, and procedure status.
- **Evidence to watch:** Institutional belief without source, records without reporter/receiver/writer, cases opened by ground truth, procedures with no lifecycle, global legal state, automated notices sourced from truth.
- **Escalation trigger:** An institution reads ground truth or creates records/sanctions without modeled report, evidence, authority, and lifecycle.
- **Retirement criteria:** `PHASE-4-ENTRY` and later institution fixtures prove wrong suspicion, false reports, stale records, failed procedures, institutional disagreement, and correction paths.
- **Status / review cadence:** Watch. Active during institution implementation and any Phase 4 entry claim.

### R-13 — Event Log, Replay, and Projection Discipline Erosion

- **Why it matters:** Event sourcing is essential, but uncontrolled event volume, poor envelope discipline, snapshots-as-truth, or weak upcasting can make replay, storage, migration, and inspection unusable.
- **Symptoms:** Every calculation becomes an event; meaningful actions lack event envelopes; replay slows before mechanics mature; snapshots erase active ancestry; old fixtures become unreadable; projection state is edited directly.
- **Mitigation / guardrails:** Commit causally meaningful events with typed causes, content versions, random draw labels, and projection hints where useful. Use snapshots, summary events, projection rebuilds, event versioning, and upcasters without losing active ancestry.
- **Evidence to watch:** Events per simulated day, replay time, projection rebuild time, old-fixture replay failures, missing causal references after compaction, active records or beliefs pointing to summarized-away detail.
- **Escalation trigger:** Authoritative state cannot be rebuilt loudly and deterministically from event log, content versions, schemas, snapshots, upcasters, and random streams.
- **Retirement criteria:** `REPLAY`/`SPINE-CERT` evidence plus deterministic replay, projection rebuild, migration, and old-fixture compatibility tests for every accepted event family.
- **Status / review cadence:** Watch. Review at every event, replay, storage, snapshot, or projection change.

### R-14 — Causal Explainability Opacity

- **Why it matters:** Tracewake's event log must answer why. IDs without useful causes cannot explain action availability, wrong beliefs, traces, records, random branches, or consequences.
- **Symptoms:** Events have IDs but weak cause links; debug answers are prose guesses; beliefs lack provenance; wrong suspicion cannot be traced; random branches are unexplained; rejected actions leave no recoverable validation report.
- **Mitigation / guardrails:** Use typed causal links, validation reports, belief sources, trace origins, record provenance, random-draw audit labels, projection rebuild reports, sealed context IDs, and debug queries shaped around “what caused this?” and “who knows what?”
- **Evidence to watch:** Events missing causes, orphan beliefs/records/traces, debug `unknown` answers, unqueryable validation reports, fixtures that cannot explain failure or contradiction cases.
- **Escalation trigger:** A gate or phase claim depends on causal behavior that cannot be explained from events, context packets, validation reports, projections, and source-backed beliefs.
- **Retirement criteria:** Golden fixtures include expected causal explanations and fail when provenance is missing.
- **Status / review cadence:** Watch. Review whenever a new event family, belief source, trace type, record type, context packet, or projection is added.

### R-15 — TUI-First Playability Erosion

- **Why it matters:** The TUI is the first playable surface and early acceptance harness. If it becomes secondary, implementation drifts toward headless simulation, graphical ambition, or debug-only play.
- **Symptoms:** Mechanics ship without TUI or view-model tests; debug commands substitute for embodied actions; future graphical assumptions shape data; the terminal UX cannot explain available actions, uncertainty, why-not reasons, or actor-known leads.
- **Mitigation / guardrails:** Require semantic TUI action selection, actor-filtered view models, actor notebook, why-not explanations, transcript replay, debug/embodied separation, and playable or view-model-testable access in every runnable phase.
- **Evidence to watch:** Headless features marked accepted, no stable semantic action IDs, UI code enforcing world rules, acceptance demos that require debug mode, user-visible summaries sourced from truth.
- **Escalation trigger:** A feature is considered accepted while unreachable or uninspectable through embodied TUI/view-model surfaces appropriate to its gate.
- **Retirement criteria:** TUI/view-model acceptance coverage for all ordinary-play features and stable boundaries for future clients.
- **Status / review cadence:** Watch. Review every feature acceptance claim.

### R-16 — No-Human Ordinary-Life Failure

- **Why it matters:** The first proof must show ordinary life without a human controller. Without no-human life, the simulation is only a reactive interface.
- **Symptoms:** Actors freeze when unpossessed; routines exist only as backdrop; hunger, fatigue, work, storage, sleep, and social behavior do not matter; incidents require human proximity; no-human runs produce no discoverable consequences.
- **Mitigation / guardrails:** Implement needs, routines, interruptions, search, storage, work, sleep, eating, social contact, household expectations, and local reports as ordinary systems. Run no-human day fixtures from early phases.
- **Evidence to watch:** No-human event distribution, routine interruption traces, ordinary action failures, discovered absence without human intervention, records or reports created by unpossessed actors.
- **Escalation trigger:** A phase or gate asserts life, institutions, or first-proof progress without `NO-HUMAN` evidence.
- **Retirement criteria:** Sustained no-human runs produce replayable, inspectable ordinary-life traces and first-proof-relevant consequences under `ORD-LIFE-CERT` and `FIRST-PROOF-CERT`.
- **Status / review cadence:** Watch. Active during routine and institution phases.

### R-17 — Agent Competence or Planner Explosion

- **Why it matters:** Tracewake needs ordinary competence without opaque or unbounded planning. Actors must act under partial knowledge, needs, roles, routines, relationships, and constraints.
- **Symptoms:** Lifeless routines; every actor replans every tick; utility jitter; planner time dominates; plans mutate state directly; agents know truth for convenience; failed actions produce no fallback; actors cannot lie, refuse, report, search, or adapt.
- **Mitigation / guardrails:** Use durable intentions, needs, projects, roles, relationships, bounded local planning, symbolic methods, planner budgets, fallback actions, reservations, and planner traces. Keep cognition inspectable and actor-known.
- **Evidence to watch:** Planner failures, stuck actors, repeated rejection storms, excessive planning nodes, choices with no belief or motive source, routines with no interruption or failure modes, candidate generation without a sealed context.
- **Escalation trigger:** Planner behavior becomes either inert backdrop or opaque high-cost authority that bypasses holder knowledge and action validation.
- **Retirement criteria:** Performance budgets, planner trace fixtures, failure/fallback tests, routine interruption tests, and `TFW`/`DIAG` evidence.
- **Status / review cadence:** Watch. Active during routine/planner implementation.

### R-18 — Schema and Authoring Drift

- **Why it matters:** Tracewake depends on authored possibility space. If schemas drift, writers and programmers will smuggle in scripts, synonyms, hidden truth, and non-replayable facts.
- **Symptoms:** Inconsistent names for the same concept; records without author/provenance; beliefs without sources; content packages bypass validation; fixture setup forces outcomes; data fields encode player identity, true culprit for institutions, objective completion, or hidden planner targets; fixture labels become holder knowledge; display totals become custody truth; domain-pack assumptions hide in prose; compiler defaults select temporal or quantity semantics; staged-abstraction fields certify omitted behavior; schemas permit ambiguous representation where execution requires fail-closed proof.
- **Mitigation / guardrails:** Keep logical contracts ahead of syntax. Validate stable IDs, references, source-backed beliefs, sealed-context provenance inputs, record artifacts, action-registry parity, no human-controller fields in world logic, no quest fields, no outcome chains, content-version compatibility, explicit temporal and quantity authority, staged-abstraction honesty, and fail-closed schema representation where execution requires proof.
- **Evidence to watch:** Schema migrations without old fixtures, content examples containing forbidden core terms, direct inventory mutation in fixtures, author confusion around canonical terms, missing provenance fields, smuggling channels through fixture labels, display totals, domain-pack prose, compiler defaults, staged-abstraction fields, and ambiguous schemas.
- **Escalation trigger:** A schema or fixture can encode hidden truth, forced outcome, or privileged human involvement as ordinary play.
- **Retirement criteria:** Validation rejects forbidden ontology and fixtures prove canonical alternatives.
- **Status / review cadence:** Active during data-authoring design; Watch afterward.

### R-19 — Validation Theater

- **Why it matters:** Tests can make a bad simulation look safe if they assert happy paths, current truth, or demo behavior instead of doctrine-threatening failure modes.
- **Symptoms:** Manual demos replace no-human runs; tests check success only; institutions are tested against truth rather than institution-known context; LLM wording appears in golden tests; replay failures are tolerated; metrics are collected but ignored.
- **Mitigation / guardrails:** Require negative tests, property tests, no-human runs, deterministic replay, projection rebuild, epistemic leakage tests, possession parity, TUI/view-model tests, debug explanation assertions, and fixture variants with false/stale/contradictory information.
- **Evidence to watch:** Green builds with no wrong-belief scenario, no rejection/failure fixtures, no replay tests, tests that pass while embodied UI cannot play the feature, metrics with no threshold or review response.
- **Escalation trigger:** A phase, spec, or gate asserts acceptance without failure evidence, adversarial contamination evidence, and replayable explanation evidence.
- **Retirement criteria:** Acceptance gates fail loudly on leakage, bypass, no-human regression, replay mismatch, forbidden terms, and weak explanations.
- **Status / review cadence:** Watch. Escalate when a phase/gate asserts acceptance without negative evidence.

### R-20 — Scale Before Clarity

- **Why it matters:** Population, regional scope, procedural history, travel, and LOD can hide shallow causality. A large world that cannot explain one missing item is hollow.
- **Symptoms:** More actors, settlements, routes, factions, notices, history, procedural generation, or performance work before the first village is gate-certified; low-detail people become props; active leads depend on compacted causes; metrics emphasize population count over causal coverage.
- **Mitigation / guardrails:** Certify the small village first under the live gate sequence: replay, no-human simulation, holder-known context, records, wrong suspicion, TUI, fixtures, and debug explanation. Expand only after `FIRST-PROOF-CERT` and relevant entry gates pass.
- **Evidence to watch:** Fixture size, causal coverage, no-human event quality, first-proof certification evidence, debug explanation depth, unresolved `P0-CERT` through `FIRST-PROOF-CERT` gates.
- **Escalation trigger:** A scale or procedural-generation feature competes with first-proof certification before `PHASE-4-ENTRY` or `SECOND-PROOF-ENTRY` allows it.
- **Retirement criteria:** First proof is certified and scale features preserve all relevant gates under replay and no-human runs.
- **Status / review cadence:** Watch. Escalate before any population, region, route, or procedural-history expansion.

### R-21 — LOD Ancestry Loss

- **Why it matters:** LOD is necessary for scale, but summaries that erase active causes destroy forensic and epistemic value.
- **Symptoms:** Low-detail actors become props; summary events hide origins of beliefs, records, leads, suspicions, or procedures; demotion erases traces; promotion invents missing ancestry; compaction makes replay explanation impossible.
- **Mitigation / guardrails:** Use explicit promotion/demotion metadata or events, summary events with retained causal facts, promotable low-detail identities, active-ancestry preservation, and compaction blocks for active traces, records, suspicions, leads, procedures, and holder-known promotion inputs.
- **Evidence to watch:** Active records pointing to missing events, promoted actors lacking home/role/beliefs/projects, debug unable to explain retained/lost detail, no-human regional run divergence under replay.
- **Escalation trigger:** LOD or compaction loses ancestry needed for holder knowledge, institutional records, active leads, wrong suspicion, procedure state, or debug explanation.
- **Retirement criteria:** Promotion, demotion, summary, compaction, replay, and explanation tests cover active ancestry.
- **Status / review cadence:** Dormant until LOD work; review before summary, compaction, regional, or scale features.

### R-22 — Story Sifting Becomes Direction

- **Why it matters:** Story sifting is allowed only as retrospective observer/debug interpretation after events. If it selects culprits, paces incidents, creates evidence, alters probabilities, or repairs quiet play, it becomes a hidden director.
- **Symptoms:** Sifter output feeds scheduler, planner, resolver, or content generation; salience changes route risk; summaries spawn events; discovery is delayed for drama; evidence appears to make a situation legible; quiet periods trigger threats.
- **Mitigation / guardrails:** Make story-sifter outputs projections only. Forbid mutation authority. Keep embodied summaries actor-filtered and debug summaries non-diegetic. Test that removing the sifter leaves simulation outcomes unchanged.
- **Evidence to watch:** Salience fields consumed by world logic, pacing fields, drama-score probabilities, sifter events without underlying causes, generated summaries treated as records or actor beliefs.
- **Escalation trigger:** Any story or salience layer affects authoritative simulation outcomes.
- **Retirement criteria:** Static boundary and regression test proving story-sifter removal does not alter event log.
- **Status / review cadence:** Watch. Review before observer summaries beyond debug recap.

### R-23 — Genre and Domain Lock-In

- **Why it matters:** The first domain is a restrained test domain, not the kernel identity. Genre assumptions in core ontology will block future domains and distort ordinary-life causality.
- **Symptoms:** Kernel types for magic, monsters, adventurers, feudal law, combat classes, medieval-only currency, tavern-specific roles, or supernatural information channels; domain packs add human-only verbs; fantasy threats become default proof.
- **Mitigation / guardrails:** Keep the kernel genre-agnostic. Put setting vocabulary, bodies, institutions, technologies, special channels, routines, threats, speech style, and economy in domain packs that obey core action, belief, event, validation, and replay rules.
- **Evidence to watch:** Core schemas with domain-specific nouns, first-proof data treated as universal, supernatural or combat systems bypassing perception, traces, norms, and records.
- **Escalation trigger:** A domain-specific concept becomes required by core engine, core schema, or acceptance architecture without being a general mechanism.
- **Retirement criteria:** Domain pack boundaries and cross-domain fixture review catch kernel leakage.
- **Status / review cadence:** Dormant now; review before any new domain pack or special information channel.

### R-24 — Deferred-Term Premature Hardening

- **Why it matters:** Notices, travel, route threats, bounties, companions, proof/payment flows, regional history, LOD, and broad story-sifting are useful later, but they can drag first-proof work into quest, director, or scale-before-clarity failure.
- **Symptoms:** Notice board becomes the central task menu; bounties encode objective completion; companions become privileged party slots; travel hazards appear for pacing; payment spawns as reward; proof is a global success flag; road threats outrank missing-property proof.
- **Mitigation / guardrails:** Keep deferred terms behind the relevant execution entry gate unless a small local artifact is necessary for the first proof. Model them as records, contracts, obligations, actors, procedures, route beliefs, payments, custody transfers, and boundary events, not as task ontology.
- **Evidence to watch:** First-proof fixture scope, schema names, TUI labels, content examples, procedure statuses, notice lifecycle, companion autonomy, route-risk causes, and references to `SECOND-PROOF-ENTRY`.
- **Escalation trigger:** A deferred concept becomes central to first-proof acceptance or enters core ontology in quest-like form.
- **Retirement criteria:** First proof is certified and second-proof docs/tests preserve canonical modeling boundaries.
- **Status / review cadence:** Watch. Review whenever deferred vocabulary appears in first-proof work.

### R-25 — Terminology Drift

- **Why it matters:** Tracewake's ontology is fragile. Casual synonyms can import wrong authority, wrong viewpoint, or wrong gameplay assumptions before anyone notices.
- **Symptoms:** Core docs, schemas, tests, prompts, or code use player-centric, quest-centric, director-centric, implementation-certification, or presentation-authority terms; the same concept has multiple names; glossary labels are ignored in fixtures; review comments normalize restricted terms.
- **Mitigation / guardrails:** Use the glossary as a naming gate. Reject forbidden-as-core terms in engine, schema, fixture, event, planner, validation, and acceptance terminology. Allow player-facing-only terms only as surface labels over canonical state.
- **Evidence to watch:** Identifiers, schema fields, event names, fixture names, TUI labels, prompt templates, comments, golden-test names, and certification artifact names.
- **Escalation trigger:** A forbidden or restricted term enters core ontology, ambiguous synonyms obscure authority/source/holder knowledge/institutional status, or old “phase landed” language replaces gate evidence.
- **Retirement criteria:** Automated or review-enforced terminology checks, plus accepted replacements in docs, schemas, tests, and gate artifacts.
- **Status / review cadence:** Watch. Review during schema/content/test naming and reference-layer edits.

### R-26 — Archived Phase 3A Treated as Post-Overhaul Certification

- **Why it matters:** Archived specs `0005` through `0008` landed historical implementation work, but the live execution layer explicitly says they are not certification under the post-overhaul foundation, architecture, and execution doctrine. Treating them as certification bypasses `P0-CERT`, hides anti-contamination gaps, and prematurely opens Phase 4 or second-proof scope.
- **Symptoms:** A prompt, spec, ticket, review, README, test comment, or implementation plan says Phase 3A is complete/certified because specs `0005` through `0008` were completed; a future spec starts institutions, wrong suspicion, notices, travel, LOD, LLM surfaces, or expansion without citing the live certification sequence; audit evidence relies on historical ticket status rather than gate artifacts.
- **Mitigation / guardrails:** Cite archived specs only as historical evidence and durable idea sources where live execution promotes them. Require future specs to declare a live posture: certification, remediation, expansion after required entry gates, or documentation alignment. Before expansion, require `P0-CERT` and the appropriate sequence through `SPINE-CERT`, `EPI-CERT`, `ORD-LIFE-CERT`, `FIRST-PROOF-CERT`, `PHASE-4-ENTRY`, or `SECOND-PROOF-ENTRY` as defined by execution docs.
- **Evidence to watch:** Status tables, README doctrine status, spec preambles, issue titles, gate checklists, CI labels, comments using “landed,” “complete,” or “done” as certification, and references to archived Phase 3A files as current doctrine.
- **Escalation trigger:** Any analysis, spec, or implementation plan treats archived specs `0005` through `0008` as proof that post-overhaul certification has passed, or uses them to bypass `P0-CERT` or a later entry gate.
- **Retirement criteria:** Do not retire until sustained review tooling and documentation patterns make historical-vs-certified status unambiguous across prompts, specs, tickets, README maps, and gate artifacts. Even then, keep as Watch while archived specs remain frequently cited.
- **Status / review cadence:** Active for post-overhaul documentation realignment and future baseline audits; review at every session start and every expansion proposal.

### R-27 — Acceptance-Evidence Reachability Overstatement

- **Why it matters:** A report can claim a path was reached while the evidence came from harness fabrication, fixture setup, or an adjacent surface. That makes acceptance artifacts look stronger than the mechanics they certify.
- **Symptoms:** Acceptance reports cite terminal events, candidate goals, replay divergence, or no-human behavior without proving the path under test emitted them; conformance rows say a family is locked when the test proves one instance; generated state stands in for scheduler, planner, or action-pipeline output; pending is counted as pass; sampled evidence is described as exhaustive certification; observer-only evidence, including `EMERGE-OBS`, is treated as a gate or behavior certificate. Execution `10` owns the evidence-status rule.
- **Mitigation / guardrails:** Count only path-under-test emissions for reachability claims. Keep fabricator source bans, advance-emitted-only floors, and explicit per-family contributor ledgers. Reports should name the command and the emitting path, not just the observed state.
- **Evidence to watch:** Acceptance artifacts, conformance rows, no-human reports, generative fixtures, replay checks, archived spec references, and historical report references that cite reachability without naming the emitter, evidence status, and guard against harness fabrication. Cross-check R-26 when archive or history is used as certification evidence.
- **Escalation trigger:** Any acceptance artifact or conformance row claims reachability, replay coverage, or ordinary-life evidence while the tested path could have been bypassed or pre-seeded by the harness.
- **Retirement criteria:** Retire only for a bounded artifact family with source bans, emitted-only counters, and report checks that fail on fabricated evidence.
- **Status / review cadence:** Watch. Review at every acceptance artifact, conformance-row update, and no-human/generative evidence claim.

### R-28 — Incomplete Correction Closure

- **Why it matters:** A fix can land on the cited instance while sibling surfaces in the same defect class keep the bug. The next audit then rediscovers the same class under a different filename or consumer.
- **Symptoms:** A correction touches one consumer but not the shared classifier; a derived census is extended by hand; a convention-scoped census only covers names inside the convention while structural siblings live outside it; a CI perimeter admits the named files but drops the conditional sibling; tests assert the originally cited site while no guard enumerates the full family; a perform-work-once correction such as a triage or migration records governance machinery instead of the resulting work product.
- **Watch:** A type-convention census can look derived while remaining hand-maintained for everything outside its convention: examples include `String`-typed field scans, single-file `include_str!` perimeters, and hand-picked token subsets of a public API (`ORD-HARD-144`/`151`/`157`). Two further shapes: the *fingerprint-payload pitfall* — an integrity fingerprint computed over the parsed/re-serialized struct instead of the raw loaded bytes, so byte-level drift and secondary-file content are invisible to it (`ORD-HARD-170`); and the *positional guard* — a substring-order assertion (token A appears before token B) standing in for an arm-complete derived census, passing when a new arm, second match, or early return bypasses the ordered pair (`ORD-HARD-174`). When a correction cites a fingerprint, record the execution `10` scope explicitly: raw bytes, normalized serialization, parsed semantic content, command transcript, run seed, or replay artifact. A fingerprint proves only the scope it covers: a byte fingerprint is not semantic proof, a parsed-content fingerprint is not raw-byte stability, a transcript hash is not replay ancestry, and a run seed is not a behavior witness.
- **Mitigation / guardrails:** Treat every named defect as a family until proven otherwise. Enumerate all members, record explicit exemptions, hoist shared rules where possible, and add structural guards that derive membership instead of trusting review memory. For one-time corrective work, make the artifact delta itself the acceptance evidence.
- **Evidence to watch:** Multi-surface facts, duplicated filters, literal censuses, allowlists, mutation perimeters, projection/view-model fields, and tickets whose acceptance criteria say "all", "every", "both", or contain unresolved conditionals.
- **Escalation trigger:** A correction is marked complete without enumerating the defect class and proving each member fixed or explicitly exempted with rationale.
- **Retirement criteria:** Retire only for a specific defect family after the shared rule and family-level guard exist; the general risk remains Watch for hardening passes.
- **Status / review cadence:** Watch. Review during ticket reassessment, implementation closeout, and spec acceptance.

### R-29 — Guard Vacuity / Decorative Locks

- **Why it matters:** A lock can assert the presence, count, hash, or hand-maintained shape of an artifact while never proving the forbidden behavior would fail. Green checks then create confidence without enforcement.
- **Symptoms:** A guard checks only that a table, list, baseline, policy row, hash, or ledger entry exists; a source scan has no synthetic negative; a policy table has no behavioral caller; a mutation or census ratchet cannot fire in the actual CI path; a nonzero-witness rule compares literal declared counts instead of measured scan matches; a source-scan guard lets display or identity facts launder through intermediate bindings; both sides of an assertion read the same source; a synthetic negative is shaped like the artifact but does not route through the production scan path. Status rows, ledgers, checksums, template tables, `EMERGE-OBS` rows, archived spec/report references, and fixture artifacts are suspect when they exist without behavior witnesses.
- **presence-check fallback:** An enumerated witness repair can still become decorative when its default arm degrades to lock-name or negative-name presence instead of measured scan execution (`ORD-HARD-141`).
- **self-satisfying citation:** A content witness can be unfalsifiable when it cites the definition site of the field or artifact it claims to prove, rather than an independent write, consume, or behavior path (`ORD-HARD-142`).
- **assertion-token-count witness:** A witness repair can relocate the presence shape into counting assertion-macro tokens (`assert!`, `assert_eq!`) in the guarded test's source text — proving the assertions are written, not that the detector inspected anything; a vacuously-empty violation set leaves the tokens present and the witness pinned (`ORD-HARD-166`).
- **prose-keyed taint:** Provenance-tracking machinery can be decorative when its sensitivity rule keys on the helper body's prose tokens instead of the argument's data provenance — the laundering path it exists to catch carries none of the keyed tokens, and kill-set synthetics that co-fire on raw-token rules certify the machinery without exercising it (`ORD-HARD-167`).
- **Mitigation / guardrails:** Every new lock must include a synthetic negative or live behavior case proving it fails when the protected behavior regresses. Prefer derived membership and path-under-test execution over hand-maintained artifact assertions. For structural lock families, keep a lock-to-negative bijection census, require nonzero real witnesses for scans, and use two-sided ratchets where a floor or baseline can silently improve or weaken.
- **Evidence to watch:** Source guards, mutation baselines, policy tables, ledger parity checks, conformance rows, and acceptance reports whose assertions are only presence, shape, or count checks.
- **Escalation trigger:** Any new or modified guard has no behavioral negative, no synthetic failing case, and no explanation for why a negative cannot be built.
- **Retirement criteria:** Do not retire globally. A specific guard family may retire only after its negative cases and path-under-test evidence are durable.
- **Status / review cadence:** Watch. Review during lock-layer, mutation, census, conformance-row, and acceptance-artifact work.

**Anti-Goodhart watch note:** Observer-only emergence counters, phenomenon families, story-sifted rows, and emergence ledgers must not become seed selectors, scheduler inputs, scenario objectives, pacing knobs, difficulty targets, LOD inputs, or pass/fail thresholds. Review this relapse through R-22 (observation becomes direction), R-16 (no-human proof pressure), R-27 (evidence overstatement), R-29 (artifact-presence without behavior-witness), and execution `10` for the non-input rule.

## Epoch-2 Temporal and Completeness Relapse Notes

These notes extend existing risk memory without creating new risk identifiers. They name review triggers introduced by temporal authority, completeness routing, and staged-abstraction evidence honesty. Use the glossary terms in `02_GLOSSARY.md`; future scoped specs own concrete fixture names, thresholds, schemas, formulas, denominations, and category taxonomies.

### Temporal authority relapse cluster

- **Clock-oracle leakage:** scheduler time, event/replay time, or validator time becomes a planning premise, procedure conclusion, lead urgency, embodied-view label, speech meaning, or LOD promotion fact without holder-known or institution-known temporal claims.
- **Raw wall-clock contamination:** host time, filesystem time, transcript time, debug-panel time, or environment time is treated as simulation fact, actor knowledge, institution knowledge, or replay evidence.
- **Omniscient lateness / office-closed labels:** availability or deadline labels reflect authoritative truth rather than source-backed holder-known or institution-known procedural time.
- **UI time-acceleration leaks:** time acceleration or skip controls reveal facts, pending outcomes, schedule shape, or urgency that the possessed holder could not know through modeled channels.
- **Debug-time becoming diegetic:** validation/debug temporal panels, transcripts, and forensic timing summaries bleed into embodied views, speech, leads, records, or procedure state.
- **Silent LOD temporal fill-in:** summaries, promotion, demotion, snapshots, or compaction generate temporal claims without temporal ancestry plus information ancestry.

### Completeness relapse cluster

- **Lineage collapse:** inventory, economy, payment, debt, storage, or record convenience replaces custody lineage with a display total, losing split/merge/transfer/consume/spoil/pay/owe/store/disclose/hide/discover ancestry or actor/institution knowledge.
- **Affect as hidden-truth / decorative-meter affect:** affect values are derived from true outcomes, fixture labels, or author intent rather than modeled experience; or emotional UI flavor is prose-only and cannot explain replayable priority without selecting hidden targets.
- **Truth-cache learning:** learned expectations store fixture labels, debug outcomes, validation failures, global success rates, or omniscient classifier outputs as actor expectations without modeled observation, memory, testimony, record, or routine experience.
- **Performance pressure as invisible director:** budgets silently suppress uninteresting, remote, inconvenient, or non-player-adjacent cognition for drama or speed.
- **Budget starvation hidden by aggregate success:** aggregate success masks repeated deferral, skipped cognition, or degraded cognition for particular actors, regions, institutions, or LOD classes.
- **Emergent injustice as author prejudice:** content packs or procedures encode social harm, credibility, access, or practical bias as author verdict or hidden moral truth instead of explicit assumptions, modeled evidence paths, partial/wrong knowledge, credibility/access/procedure, and domain-pack-bound premises.
- **Play-legibility dual relapse:** one failure mode is a correct-but-unplayable audit machine; the opposite is play-legibility achieved by leaking truth, objectives, quest markers, debug labels, or omniscient summaries. Review whether play remains understandable through holder-known surfaces, leads, notices, and explainable procedure without replacing play with audit-only source inspection.

### Staged-abstraction evidence-honesty note

Acceptance artifacts can falsely certify staged work when staged abstraction is buried, omitted behavior is unnamed, future blockers are obscured, evidence scope is not tied to the abstraction, or failure diagnostics cannot distinguish "not implemented yet" from "implemented and broken." This note is additive to the existing acceptance-evidence cluster; it does not reopen, rename, or strengthen R-27, R-28, or R-29.

## Maintenance rule

Add a risk only when a recurring relapse mode needs compact memory across coding, spec writing, fixture authoring, schema review, prompt construction, and gate review. Retire a risk only with named evidence that would catch its return. Do not use this register to define new doctrine, redefine gate semantics, certify code, or create implementation work.
