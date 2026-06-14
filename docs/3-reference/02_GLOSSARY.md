# Glossary

## Status

This is Tracewake's compact terminology-control document for implementation names, documentation, schemas, fixtures, tests, prompts, TUI labels, debug tools, and review. The glossary is prescriptive. A term may be acceptable in player-facing prose while still being restricted or forbidden as engine ontology.

The glossary is now realigned to the post-overhaul spine. The system-wide canonical term is **holder-known context**. **Actor-known** remains valid as the foundation-tier and actor-specific case of that broader architecture term.

When in doubt, choose the canonical term and preserve the distinction among truth, observation, belief, memory, claim, speech act, trace, record, institutional fact, projection, debug truth, holder-known context, and surface wording.

## Authority boundary

This file owns compact naming guidance only. It does not define doctrine, architecture contracts, gate semantics, implementation plans, data schemas, or acceptance criteria. Gate codes named here are lookup labels whose definitions live in `docs/2-execution/`.

## Depends on

- `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`
- `docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md`
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
- `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md`
- `docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md`
- `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`

## Usage labels

- **Canonical** — safe as an engine, schema, fixture, test, documentation, architecture, prompt, or review term.
- **Canonical projection** — safe for derived read models; must not imply authority to mutate simulation fact.
- **Canonical, context-bound** — safe only when the context is explicit; do not use as a vague synonym.
- **Debug-only** — acceptable only in non-diegetic inspection, test, or forensic tooling.
- **Player-facing only** — acceptable in UI/prose as a surface label over canonical state; not core ontology.
- **Restricted** — allowed in narrow contexts, but dangerous enough to require review.
- **Deferred but allowed** — allowed after the phase/gate that can support it; do not harden into first-proof core unless explicitly justified.
- **Forbidden as core** — do not use as authoritative engine ontology, schema, fixture, event, planner, validation, or acceptance terminology.

## Naming rules

Use **actor** for a world entity capable of perception, belief, intention, and action. Use **human controller**, **controller binding**, and **possession** for input and viewpoint. Do not create a privileged player entity.

Use **holder** for the broad architecture role that owns a decision-facing knowledge boundary. A holder can be an actor, institution, household, role office, speaker, listener, embodied viewer, TUI affordance selector, lead interpreter, LOD promotion recipient, or regional procedure owner.

Use **holder-known context** for the system-wide sealed decision context. Use **actor-known** when the holder is specifically an actor or when quoting foundation-level actor cognition language. Use **institution-known context** when the holder is an institution or institutional procedure.

Use **truth firewall** for the boundary summarized by: “Truth may validate actions, but truth may not plan them.” Validators may read authoritative truth; planners, procedures, embodied affordance selectors, speech interpreters, institutions, leads, and promoted holders must receive only holder-known context.

Use **action proposal**, **validation report**, **schedule**, **resolution**, **event**, and **projection rebuild** for the ordinary world-affecting pipeline. Do not let clients, fixtures, prompt text, debug views, or language surfaces mutate authoritative state.

Use **lead**, **notice**, **record**, **request**, **contract**, **obligation**, **incident**, **report**, **suspicion**, **procedure**, **proof**, **payment**, or **custody transfer** instead of quest terms. These are source-bound artifacts, states, or projections, not objective task truth.

Use **speech act** for structured communication. Surface prose, deterministic templates, and LLM output may render or parse, but they do not create simulation fact.

Use **ground truth**, **observation**, **belief**, **memory**, **claim**, **record**, and **institutional fact** distinctly. A belief can be wrong. A record can be false. A procedure can be valid but unjust. An observation is not interpretation. Ground truth is not automatically known.

Use **evidence ledger**, **exact-commit source**, **manifest path inventory**, **provenance**, and **stale source** when discussing AI-session repository evidence. Do not trust branch names, connector labels, uploaded filenames, hidden memory, prior chats, or snippet search as source authority.

Use exact execution gate codes only as cross-references to `docs/2-execution`; do not redefine them in reference, specs, tickets, or comments.

## Canonical terms

| Term | Status | Definition / naming note |
|---|---|---|
| Action | Canonical | A world-affecting attempt or transition processed through the shared pipeline. It may be rejected, fail, be interrupted, or commit events. |
| Action definition | Canonical | A registered action kind with parameters, validation hooks, duration or scheduling semantics, possible outcomes, trace rules, event rules, and projection expectations. |
| Action pipeline | Canonical | The shared path from command or proposal through validation report, scheduling or resolution, event commit, projection rebuild, replay, and debug explanation. |
| Action proposal | Canonical | A candidate action submitted by a human controller, agent, institution procedure, language parser, fixture harness, or debug/test tool. It is not authority and may be rejected. |
| Action registry | Canonical | The catalog of action definitions available to ordinary actors and processes. It supports parity, validation, schemas, TUI affordances, and testing. |
| Actor | Canonical | A world entity capable of perception, belief, intention, and action. A human may temporarily control an actor, but the actor remains ordinary. |
| Actor-known | Canonical | Foundation-tier name for what is available to a specific actor through observation, memory, belief, speech, records read, traces observed, or modeled inference. Actor-known is the actor case of the broader architecture term holder-known context. Actor-known does not mean true. |
| Actor notebook | Canonical projection | A source-bound projection of one actor's beliefs, memories, observations, contradictions, records read, speech heard, and actor-known leads. It excludes hidden truth and debug notes. |
| Affordance | Canonical | A typed possible interaction exposed by an entity, place, role, record, social context, or process. It proposes candidate actions; it does not grant success, permission, or truth. |
| Agent | Canonical | The decision-making system associated with an actor. It reads actor-specific holder-known context and proposes actions; it does not commit authoritative state directly. |
| Agent detail tier | Canonical | A declared level of cognitive or simulation fidelity for an actor. Lower detail may summarize but must preserve promotability and necessary ancestry. |
| AI-session context | Canonical, context-bound | The non-authoritative working context of an AI assistant session. It must be refreshed from exact sources and must not override fetched evidence. |
| Authoritative simulation | Canonical | The kernel-owned world authority: validated actions, event log, replay, current-state projections, and causal rules. UI, prose, LLMs, debug views, and story summaries are not substitutes. |
| Belief | Canonical | A holder-specific proposition with stance, confidence, source, acquisition time, and possible contradictions. It may be true, false, stale, speculative, private, or socially influential. |
| Belief holder | Canonical | The actor, institution, household, role, or other holder that holds a belief. Beliefs without holders are invalid. Prefer holder when the decision boundary is broader than belief storage. |
| Belief source | Canonical | The observation, memory, speech act, record, inference, trace, routine expectation, or procedure that supports a belief. |
| Belief store | Canonical projection | A rebuildable projection of source-backed beliefs by holder, proposition, stance, confidence, contradiction, and time. |
| Boundary event | Canonical | An event entering the simulated region from outside with summarized causes, such as migration, weather, disease, market pressure, order, tax demand, caravan arrival, or route report. |
| Causal ancestry | Canonical | The preserved chain of causes needed to explain a state, belief, record, trace, lead, suspicion, procedure, projection, or promoted holder-known fact after replay, compaction, or LOD change. |
| Causal graph | Canonical | Queryable links among events, intentions, observations, beliefs, traces, records, norms, validation reports, random branches, context packets, and consequences. It supports explanation and replay. |
| Claim | Canonical | A proposition asserted, implied, recorded, reported, remembered, or rendered by some source. A claim is not automatically true. |
| Command | Canonical | Boundary input from a human controller, TUI, agent planner, institution procedure, language surface, debug tool, test harness, or other process. A command is a proposal, not a fact. |
| Component | Canonical | A structured piece of entity state owned by the authoritative simulation and changed only by event application or explicit setup/test mechanisms. |
| Contamination gate | Canonical, context-bound | A review or acceptance check, especially execution `TFW`, proving hidden truth, validator truth, fixture truth, debug truth, player-only state, and prose output do not feed holder cognition/procedure/view decisions. Gate semantics live in execution docs. |
| Contamination review | Canonical, context-bound | The audit of whether a decision, proposal, fixture, view model, prompt packet, or diagnostic was influenced by forbidden context sources. It should name the holder, sealed context, provenance classes, and excluded truth. |
| Context sealing | Canonical | The act of freezing a holder-known context packet before a decision. After sealing, validation failure or debug comparison may create future modeled feedback, but it cannot retroactively add truth to the failed decision. |
| Content package | Canonical | Versioned authored content that defines possibility space: entities, places, actions, norms, routines, records, fixtures, domain vocabulary, and validation rules. It must not force hidden ordinary-play outcomes. |
| Content version | Canonical | A durable identifier for the content package or schema set used by replay, validation, save packages, and fixture compatibility checks. |
| Contract | Canonical | A formal or semi-formal obligation among parties, with terms, authority, evidence, proof, payment, sanctions, dispute, and lifecycle where modeled. It is not a quest. |
| Controller binding | Canonical | The non-diegetic mapping from human controller input to the currently possessed actor and view. It is not world identity or actor knowledge. |
| Custody | Canonical | Responsibility for holding, guarding, storing, or managing an item or record, distinct from legal ownership and current physical possession. |
| Debug injection | Debug-only | An explicit test/debug event or setup operation used to create, repair, or inspect state outside ordinary play. It must be marked and must not masquerade as an ordinary action. |
| Debug mode | Debug-only | Non-diegetic inspection mode that may reveal truth, event logs, causal graph, hidden beliefs, validation reports, projection diffs, and replay diagnostics. It must not feed embodied actor knowledge. |
| Debug truth | Debug-only | Omniscient or near-omniscient information exposed only for inspection, testing, or forensic analysis. It is not actor knowledge, institution-known context, or player-facing fact. |
| Deterministic mock | Canonical, context-bound | A controlled replacement for nondeterministic external behavior, especially language generation, used for tests and acceptance. It does not relax validation or authority boundaries. |
| Domain pack | Canonical | Scenario or setting content layered over the genre-agnostic core: bodies, institutions, norms, items, technologies, routines, special channels, speech style, threats, and fixtures. |
| Embodied mode | Canonical | Normal play mode in which the human sees and acts through the possessed actor's accessible perception, beliefs, memories, records, affordances, and why-not explanations. |
| Entity | Canonical | A persistent world object, actor, place, institution, artifact, or process identity tracked by authoritative state and events. |
| Evidence ledger | Canonical, context-bound | The AI-session source record listing requested repository, target commit, manifest role, fetch method, exact fetched URLs, prohibited methods avoided, contamination observed, and freshness claim. |
| Evidence threshold | Canonical | The amount and kind of institution-known testimony, trace, record, authority, role, or procedure required before an institution may classify, prove, sanction, post, close, or pay. |
| Event | Canonical | An immutable record of a meaningful occurrence, attempted action, failed action, observation, belief update, speech act, trace, record change, schedule change, or state mutation. |
| Event envelope | Canonical | The event wrapper containing identity, type, version, time/order, streams, actors/processes, location, causes, random draws, effects, content version, and replay/debug metadata as needed. |
| Event log | Canonical | The ordered authoritative history from which state, beliefs, records, projections, view models, and debug explanations can be rebuilt. |
| Event sourcing | Canonical | The architecture where meaningful change is stored as ordered events and current state is rebuilt through replay and projections. |
| Exact-commit source | Canonical, context-bound | Repository content fetched from a URL containing the requested owner, repository, full commit hash, and manifest path. Anything else is not exact-source evidence. |
| Exogenous boundary event | Canonical | Same as boundary event when emphasizing outside-region causes and summarized provenance. |
| Failure event | Canonical | An event recording that an attempted action failed or was interrupted after entering resolution. It differs from validation rejection. |
| Fixture | Canonical | A controlled content/test setup for proving behavior, replay, validation, projections, or regression boundaries. It may seed initial state but must not script ordinary-play outcomes. |
| Fixture variant | Canonical | A deliberate variation of a fixture that exercises false beliefs, stale records, alternative observations, failure paths, possession changes, no-human runs, or hidden-truth contamination cases. |
| Gate code | Canonical, context-bound | A compact execution-layer label such as `P0-CERT` or `TFW`. Use exact codes only as cross-references; definitions and pass/fail semantics live in `docs/2-execution/`. |
| Ground truth | Canonical | The authoritative world state and event history. Actors, institutions, embodied UI, language surfaces, records, and holder-known contexts do not automatically know it. |
| Holder | Canonical | Any entity, office, role, surface, or process that makes cognition-like or procedure-like decisions. Architecture names actors, institutions, households, role offices, speakers, listeners, embodied viewers, TUI affordance selectors, lead interpreters, LOD promotion recipients, and regional procedure owners. |
| Holder-known context | Canonical | The system-wide canonical term for a decision-facing, holder-specific context built from modeled sources before cognition, procedure choice, speech interpretation, affordance selection, lead interpretation, or LOD promotion. It is fallible, source-backed, sealed for the decision, and not ground truth. |
| Human controller | Canonical | The non-diegetic input source that may bind to an actor. It is not an actor, not a privileged identity, and not world truth. |
| Incident | Canonical | Something that happened or is believed to have happened and may matter to actors or institutions. It may remain private, be misreported, become stale, or never involve the human controller. |
| Institution | Canonical | A fallible social machine made of actors, roles, records, resources, norms, procedures, jurisdiction, authority, bias, and failure modes. It acts from institution-known context, not ground truth. |
| Institution-known context | Canonical | The holder-known context for an institution or institutional procedure: records, reports, roles, jurisdiction, resources, procedure state, public artifacts, institutional memory, evidence thresholds, and uncertainty available through modeled sources. It is not omniscient truth. |
| Institutional fact | Canonical | A fact created by valid institutional procedure, such as a received report, ledger entry, posted notice, order, custody transfer, payment authorization, or case closure. It can be wrong, forged, stale, or contested. |
| Institutional knowledge | Canonical | The beliefs, records, reports, notices, roles, procedure states, and remembered claims available to an institution through modeled sources. Prefer institution-known context when naming the sealed decision packet. |
| Intention | Canonical | A durable commitment by an agent to pursue a project, routine, need response, obligation, or plan until completion, failure, interruption, abandonment, or replacement. |
| Knowledge context | Canonical | The explicit holder and access boundary used when querying beliefs, view models, planner inputs, prompt packets, records, or institutions. Prefer holder-known context for sealed decision packets; knowledge context remains valid for APIs or projections that name holder/access scope. |
| Lead | Canonical projection | A holder-facing, source-bound projection of possible significance. A lead may suggest actions but does not imply truth, solvability, intended human involvement, completion, or payment. |
| Level of detail / LOD | Canonical | Explicit simulation fidelity for actors, places, regional processes, and history. Lower detail summarizes but must preserve necessary causal ancestry and promotability. |
| Manifest path inventory | Canonical, context-bound | A user-supplied list of repository paths allowed or required for exact URL fetches. It is not branch evidence, freshness evidence, or file content. |
| Memory | Canonical | Actor-specific retained information shaped by source, salience, recency, emotion, repetition, contradiction, relationship, role, and possible forgetting or distortion. |
| Need | Canonical | A pressure such as hunger, fatigue, safety, shelter, sleep, social standing, or work obligation that influences perception, willingness, planning, and action selection. It does not grant truth. |
| No-human simulation | Canonical | Running the world without a human controller bound to any actor. It proves ordinary systems do not depend on possession or human proximity. |
| Norm | Canonical | A social or institutional rule such as obligation, prohibition, permission, privacy rule, status rule, evidence threshold, jurisdiction rule, or procedure rule. Norms can be violated. |
| Notice | Canonical | A physical or public information artifact with issuer, author or scribe, structured claims, source, location, readers, lifecycle, and stale risk. It is not a task-menu item. |
| Observation | Canonical | Channel-specific perception or discovery by an actor or process. Observation is not interpretation and does not automatically transfer truth. |
| Ordinary actor | Canonical | Any simulated actor with normal constraints: needs, beliefs, access, roles, relationships, intentions, fallibility, and action parity. A possessed actor is still ordinary. |
| Possession | Canonical | Non-diegetic binding of a human controller to an actor for input and embodied view. It changes control and viewpoint, not truth, knowledge, identity, memory, guilt, or privilege. |
| Procedure | Canonical | A structured institutional or social process with authority, roles, evidence, steps, possible failure, status, records, and outcomes. It is not a script. |
| Procedure status | Canonical | The current modeled state of a procedure, such as opened, awaiting report, gathering evidence, disputed, proven by threshold, sanctioned, paid, closed, stale, or abandoned. |
| Projection | Canonical | A derived, rebuildable read model from events and content versions, such as current state, belief store, actor notebook, ledger view, lead card, TUI view, or debug graph. |
| Projection rebuild | Canonical | Recomputing a projection from event log, content versions, schemas, snapshots, and upcasters. Rebuild mismatches must fail loudly in authoritative contexts. |
| Proposition | Canonical | A structured claim that can be believed, reported, recorded, inferred, contradicted, remembered, or used in speech. A proposition is not automatically true. |
| Provenance | Canonical | Source and causal origin. Applies to repository evidence, events, beliefs, records, traces, projections, context packets, generated summaries, random draws, and institutional knowledge. |
| Provenance class | Canonical | A typed category for how a context item is known: direct observation, search result, uncertain sound, absence observation, memory, testimony, written record, public artifact, household/institutional record, assignment, role/procedure state, public notice, rumor, lead, LOD summary event, or explicit unknown. |
| Random draw | Canonical | A seeded or recorded nondeterministic choice used by simulation. It must be replayable or auditable according to phase and feature needs. |
| Random-draw audit label | Canonical | A stable label explaining what a random draw was for, so replay and debug can explain not only the value but the decision point. |
| Record | Canonical | A durable world or institutional artifact containing structured claims, author or issuer, physical or storage location, source, readers, amendments, and lifecycle events. |
| Replay | Canonical | Rebuilding authoritative state and projections from event log, content versions, schemas, snapshots, upcasters, and random streams. Authoritative replay fails loudly on mismatch. |
| Replay checkpoint | Canonical | A saved replay boundary, snapshot, or diagnostic point used to resume, compare, or audit replay without losing event authority or active ancestry. |
| Report | Canonical | A speech act, record entry, or institutional submission in which a holder communicates claims to another actor or institution. It may be wrong, incomplete, stale, biased, or false. |
| Routine | Canonical | A recurring, defeasible intention pattern such as sleeping, eating, working, office hours, patrol, storage, social visit, or household maintenance. Routines have preconditions, duration, interruptions, and failure modes. |
| Salience | Canonical, context-bound | Observer-layer or memory-layer importance used to surface, summarize, retain, or inspect chains. Salience may highlight; it must not cause world events. |
| Scenario seed | Canonical | Authored initial tensions, placements, relationships, expectations, records, memories, and pressures for a scenario. It seeds possibility; it does not script outcomes. |
| Scheduler | Canonical | The deterministic simulation-time system for wakeups, action durations, ordering, reservations, interruptions, no-human stepping, time acceleration, and replay. |
| Sealed context packet | Canonical | The immutable holder-known packet used for one decision. It includes holder identity/kind, trigger, tick/window, claims, observations, memories, records/artifacts, roles, resources, procedure/intention state, uncertainty, explicit unknowns, provenance edges, forbidden-truth audit, and a context ID/hash. |
| Schema | Canonical | A versioned content or event contract with validation, stable IDs, source rules, references, compatibility, and migration expectations. A schema is not a script. |
| Source-backed belief | Canonical | A belief with an explicit holder, proposition, source, acquisition time, confidence or stance, and contradiction handling. Beliefs without sources are invalid except explicit test/debug setup. |
| Speech act | Canonical | A typed communication action such as ask, answer, report, accuse, lie, gossip, promise, refuse, testify, recruit, threaten, confess, or request. It carries propositions, speaker stance, listeners, and validation. |
| Stale information | Canonical | Information that may once have been accurate but may no longer match ground truth. Track source, event time, acquisition time, last verification, confidence, and contradiction. |
| Stale source | Canonical, context-bound | Repository, document, AI-session, manifest, or research context that may be outdated relative to the requested target. It must be quarantined until exact-source provenance is re-established. |
| Story sifting | Canonical projection | Observer/debug grouping or summarization of event chains after they occur. It may highlight patterns and contradictions but may not direct, spawn, pace, or repair events. |
| Suspicion | Canonical | A holder-specific or institution-specific belief that an actor, place, object, route, or circumstance may be implicated. It requires source and uncertainty and may be wrong. |
| Trace | Canonical | A physical, mental, social, institutional, economic, environmental, or erasure residue left by an event that can later influence observation, belief, records, or action. |
| Truth firewall | Canonical | The boundary expressed by “Truth may validate actions, but truth may not plan them.” Authoritative truth may validate, resolve, mutate through events, generate modeled observations, and support debug comparison; it may not feed cognition/procedure except through modeled holder-known channels. |
| TUI | Canonical | The terminal-first playable client and acceptance surface. It consumes actor-filtered view models and submits commands; it is not a rules engine. |
| Upcaster | Canonical | A version-aware migration mechanism that lets old events, content, or save packages replay under newer schemas without silently rewriting history. |
| Validation gate | Canonical, context-bound | A general proof checkpoint. Prefer exact execution gate codes when discussing live certification; do not use this term to redefine `docs/2-execution` semantics. |
| Validation report | Canonical | The structured result of validating a proposal: accepted, rejected, blocked, risky, transformed, scheduled, or failed with actor-filtered why-not reasons and debug-level details where appropriate. |
| View model | Canonical | A projection shaped for a consumer such as embodied TUI, debug UI, notebook, record view, report view, or future graphical client. Embodied view models are actor-filtered. |
| Why-not UI | Canonical | Actor-filtered explanation of why an action is unavailable, risky, blocked, rejected, or differently classified, using state, belief, norm, resource, role, duration, or precondition reasons. |
| World-affecting change | Canonical | Any change to authoritative world state, belief state, records, traces, schedules, procedures, custody, institutional facts, or replay-relevant history. It must pass through the proper authority path. |

## Canonical context-bound and deferred terms

| Term | Status | Use boundary |
|---|---|---|
| Bounty | Deferred but allowed | Later public contract, obligation, report, notice, procedure, or payment structure. It is not a task menu, objective, or guaranteed reward. |
| Companion | Deferred but allowed | Later autonomous actor relation such as recruited actor, co-traveler, escort, follower, contract party, or helper with needs, beliefs, refusal, and independent action. |
| `EMERGE-OBS` | Canonical, context-bound | Execution-layer observation-obligation lookup label for observer-only emergence evidence. Semantics live in `docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md` and `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`; this glossary entry defines no gate. |
| Evidence | Canonical, context-bound | A trace, observation, report, record, testimony, artifact, contradiction, or institutional source used by a holder or procedure. Evidence is holder-relative unless explicitly debug-only. |
| Landed historical spec | Canonical, context-bound | A completed archived spec or ticket can be historical evidence. It is not live certification unless a live gate artifact says so. |
| Notice board | Restricted | A place or artifact that displays notices. It must not become a task menu. Each notice still needs issuer, source, claims, readers, lifecycle, and stale risk. |
| Observer-only emergence evidence | Canonical, context-bound | Canonical name for the after-the-fact artifact used to document emergent ordinary-life outcomes under `INV-111`, and the glossary class name for architecture 13's observer-only emergence-evidence record. It is retrospective, non-certifying, ancestry-backed, non-steering, and structurally non-input: produced from completed event-log/replay ancestry, never enough alone to pass behavior gates, tied to causal event-log ancestry, replay/extraction provenance, and the phenomenon-family row contract, and never read by cognition, scheduler, validators, authoring, seed/scenario selection, pacing, LOD, difficulty, outcome gates, simulation state, planner data, content-selection data, or pass/fail thresholds. It is not ordinary evidence, observation, projection, or story sifting; governing sources are foundation `02`/`09`/`12`, architecture `13`, and execution `00`/`10`. |
| Ownership | Canonical, context-bound | A normative or institutional relation distinct from custody, physical possession, use right, household expectation, theft suspicion, and record state. |
| Payment | Canonical, context-bound | A modeled transfer, authorization, debt settlement, wage, fee, compensation, or procedure outcome. It is not a spawned completion reward. |
| Phase label | Restricted | Historical locator such as Phase 1, Phase 2A, Phase 3A, or Phase 4. It is not a live certification claim. Use execution gate codes for certification posture. |
| Proof | Canonical, context-bound | An institutional or social threshold outcome based on evidence, authority, and procedure. It is not ground truth and not a universal success flag. |
| Request | Canonical, context-bound | A speech act, record, obligation, or social pressure asking someone to act. It does not imply acceptance, success, completion, or payment. |
| Route threat | Deferred but allowed | Later route-level danger or uncertainty produced by modeled causes, reports, traces, regional processes, or boundary events. It is not a pacing device. |
| Travel | Deferred but allowed | A spatial action family involving route knowledge, time, risk, resources, companions, traces, observations, and boundary events. Broad route play belongs after the first proof. |

## Restricted and forbidden-as-core terms

| Term | Status | Use instead / danger |
|---|---|---|
| Accepted quest | Forbidden as core | Use obligation accepted, contract formed, report received, procedure opened, promise made, payment authorized, or actor intention. |
| AI director | Forbidden as core | Use story sifting only for retrospective projection. Director language implies hidden pacing authority over causality. Acceptable only when naming a counterexample. |
| Certified Phase 3A | Restricted | Use only if a live gate artifact actually certifies the relevant execution gates. Historical specs `0005` through `0008` landed; they are not post-overhaul certification by themselves. |
| Clue | Player-facing only | Use trace, observation, lead, record claim, rumor, testimony, contradiction, or evidence. Clue implies an artifact exists for a human solver. |
| Completion | Forbidden as core | Use world state, actor belief, record status, procedure status, obligation fulfillment, custody transfer, payment status, or institutional decision. |
| Completion flag | Forbidden as core | Same danger as Completion. Different actors and institutions may disagree about whether anything is resolved. |
| Cutscene truth | Forbidden as core | Use event, observation, record, actor-known summary, or debug summary. Presentation cannot declare world truth. |
| Drama director | Forbidden as core | Same danger as AI director: pacing, excitement, or narrative need starts causing events. |
| Global wanted meter | Forbidden as core | Use reports, records, suspicion, testimony, proof thresholds, institutional knowledge, notices, sanctions, and jurisdiction. Global meters imply omniscient law. |
| Ground-truth dialogue | Forbidden as core | Use speech act backed by speaker belief, lie, speculation, question, record, observation, or debug-only summary. Dialogue text cannot smuggle truth. |
| Hallucinated fact | Forbidden as core | Reject, repair, or classify as unsupported output. It is not a lie unless an actor intentionally performs a validated lie from a modeled motive/context. |
| Map marker | Restricted | Use actor-known place belief, route lead, record location, visible landmark, or debug marker. Embodied markers must not point to hidden truth. |
| Narrative beat | Restricted | Use incident chain, event cluster, salience group, or story-sifting projection. Do not let beat structure schedule or repair world events. |
| NPC | Player-facing shorthand only | Use actor or agent in core docs, schemas, tests, prompts, and code. This term frames the world around a player/non-player split. |
| Objective | Forbidden as core | Use intention, need, project, obligation, procedure step, actor-known lead, or validation target depending on context. Objective implies intended solution and privileged truth. |
| Objective marker | Forbidden as core | Use actor-known place belief, route belief, record claim, affordance hint, or debug marker. It must not point to hidden truth. |
| Omniscient actor | Forbidden as core | Use actor with beliefs, observations, memory, sources, inference, access, and fallibility. No ordinary actor reads ground truth. |
| Party inventory | Forbidden as core | Use custody, possession, storage, contract terms, carried item, household property, or institutional custody. Shared party inventory creates privileged group abstraction. |
| Party member | Restricted | Use companion, recruited actor, co-traveler, escort, contract party, follower, or helper with autonomy, needs, beliefs, refusal, and independent action. |
| Player | Restricted | Use human controller for input source and actor for world entity. Player-facing is acceptable when discussing UI surface wording. |
| Player character | Forbidden as core | Use actor for the world entity and human controller for input source. Player character implies a privileged world identity. |
| Protagonist | Forbidden as core | Use possessed actor or currently controlled actor only when discussing viewpoint. Protagonist implies world-centered narrative importance. |
| Quest | Forbidden as core | Use incident, request, contract, obligation, notice, lead, record, suspicion, report, or procedure. Quest implies objective task truth, intended human involvement, completion, reward, and waiting world state. |
| Quest giver | Forbidden as core | Use requester, reporter, issuer, contract party, clerk, witness, authority, notice author, or procedure participant. |
| Quest target | Forbidden as core | Use actor-known lead target, reported location, suspected actor, record claim, search area, obligation object, or procedure subject. |
| Reward | Restricted | Use payment, obligation, custody transfer, gift, debt settlement, wage, sanction relief, compensation, or institutional authorization. Reward implies spawned payoff for completion. |
| Script | Forbidden as core for ordinary play | Use action definition, routine, procedure, scenario seed, fixture setup, boundary process, or validation harness. Script implies forced outcome sequence. |
| Scripted event | Forbidden as core | Use event generated by validated action or modeled process. Fixture setup may create initial state, but normal play cannot rely on hidden scheduled outcomes. |
| Solve | Player-facing only | Use recover, report, prove, close record, change belief, return property, satisfy obligation, settle debt, transfer custody, or update procedure. Situations resolve differently by truth, actor belief, and institutional state. |
| Spawn | Restricted | Use boundary event, birth, arrival, placement, creation event, fixture setup, process output, or generated content with provenance. Spawn often hides cause and source. |
| Storyteller | Forbidden as core | Acceptable only in comparative discussion. Tracewake observes story after causality; it does not author events for story shape. |
| Success state | Restricted | Use validation accepted, action resolved, event committed, procedure status, obligation fulfilled, record closed, payment authorized, or belief changed. Success must name the layer. |
| Task board | Restricted | Use notice board, record display, request list, procedure queue, or actor-known lead list. It must not imply objective menu selection. |

## Maintenance rule

Keep this glossary compact and prescriptive. Add a term only when repeated coding, schema, fixture, prompt, review, or TUI usage needs a stable boundary. Do not use glossary entries to smuggle doctrine, architecture, gate semantics, or implementation plans into reference. If a term requires expanded policy, move that work to the proper higher layer.
