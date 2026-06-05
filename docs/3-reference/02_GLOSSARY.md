# Glossary

## Status

This is Tracewake's compact terminology-control document. It is for implementation names, documentation, schemas, tests, prompts, TUI labels, debug tools, and code review.

The glossary is somewhat prescriptive. A term may be acceptable in player-facing prose while still being forbidden as engine ontology. When in doubt, prefer the canonical term and preserve whether something is truth, belief, record, projection, or surface wording.

## Usage labels

- **Canonical** — safe as an engine, schema, test, documentation, or architecture term.
- **Canonical, context-bound** — safe when the context is explicit; do not use as a vague synonym.
- **Player-facing only** — acceptable in UI/prose as a label over canonical state, not as core ontology.
- **Debug-only** — acceptable only in non-diegetic inspection tools.
- **Forbidden as core** — do not use as authoritative engine ontology, schema, fixture, event, planner, or validation terminology.

## Naming rules

Use **actor** for a world entity capable of perception, belief, intention, and action. Use **human controller** and **possession** for player input binding. Do not create a privileged player entity.

Use **lead**, **notice**, **record**, **request**, **contract**, **obligation**, **incident**, or **suspicion** instead of quest terms. These are source-bound world artifacts or projections, not objective task truth.

Use **speech act** for structured communication. Surface text, template output, or LLM prose does not create simulation fact.

Use **belief**, **observation**, **memory**, **record**, and **ground truth** distinctly. A belief can be wrong. A record can be false. An observation is not interpretation. Ground truth is not automatically known.

## Canonical terms

| Term | Status | Definition / naming note |
|---|---|---|
| Action | Canonical | A validated world-affecting attempt or transition performed by an actor or process through the shared pipeline. An action may be rejected, fail, be interrupted, or commit events. |
| Actor | Canonical | A world entity capable of perception, belief, intention, and action. A human may temporarily control an actor, but the actor remains ordinary. |
| Actor-known | Canonical | Available to a specific actor through perception, memory, belief, speech, records read, traces observed, or modeled inference. Not the same as true. |
| Actor notebook | Canonical | A projection of one actor's source-bound beliefs, memories, observations, contradictions, records read, speech heard, and actor-known leads. It excludes hidden truth and debug notes. |
| Affordance | Canonical | A typed possible interaction exposed by an entity, place, role, record, social context, or process. It proposes candidate actions; it does not grant success or permission. |
| Agent | Canonical | The decision-making system associated with an actor. It reads actor-specific beliefs and proposes actions; it does not commit authoritative state directly. |
| Authoritative simulation | Canonical | The kernel-owned world authority: validated actions, events, replay, current-state projections, and causal rules. UI, prose, LLMs, and debug views are not substitutes for it. |
| Belief | Canonical | A holder-specific proposition with stance, confidence, source, acquisition time, and possible contradictions. It may be true, false, stale, speculative, private, or socially influential. |
| Causal graph | Canonical | Queryable links among events, intentions, beliefs, observations, traces, records, norms, random branches, and consequences. It supports explanation, replay, and validation. |
| Command | Canonical | An input or request from a boundary such as human controller, TUI, agent planner, institution procedure, language surface, or debug tool. A command is a proposal, not a world fact. |
| Contract | Canonical | A formal or semi-formal obligation among parties, with terms, authority, evidence, proof, payment, sanctions, dispute, and lifecycle where modeled. It is not a quest. |
| Custody | Canonical | The actor, institution, household, or object responsible for holding or managing an item, distinct from legal ownership and current physical possession. |
| Debug mode | Debug-only | Non-diegetic inspection mode that may reveal truth, event logs, causal graph, hidden beliefs, validation reports, and projection diffs. It must not feed embodied actor knowledge. |
| Domain pack | Canonical | Scenario or setting content layered over the genre-agnostic core: bodies, institutions, norms, items, technologies, special channels, routines, speech style, threats, and fixtures. |
| Embodied mode | Canonical | Normal play mode in which the human sees and acts through the currently possessed actor's accessible perception, beliefs, memories, records, affordances, and why-not explanations. |
| Event | Canonical | An immutable record of a meaningful occurrence, attempted action, failed action, observation, belief update, speech act, trace, record change, schedule change, or state mutation. |
| Event sourcing | Canonical | The architecture where meaningful state change is stored as ordered events and current state, records, beliefs, leads, and view models are rebuildable projections. |
| Exogenous boundary event | Canonical | An event entering the simulated region from outside, with summarized causes, such as migration, caravan arrival, weather, disease, war order, market pressure, or tax demand. |
| Ground truth | Canonical | The authoritative world state and event history. Actors, institutions, embodied UI, and language surfaces do not automatically know it. |
| Household | Canonical | A domestic institution with members, roles, property expectations, storage, food access, beds, privacy, obligations, permissions, memory, secrets, and expected presence. |
| Human controller | Canonical | The non-diegetic input source that may bind to an actor. It is not an actor, not a player character, and not world truth. |
| Incident | Canonical | Something that happened or is believed to have happened and may matter to actors or institutions. It may remain private, be misreported, become stale, or never involve the human. |
| Institution | Canonical | A fallible social machine made of actors, roles, records, resources, norms, procedures, jurisdiction, authority, bias, and failure modes. It acts from institutional knowledge, not truth. |
| Institutional fact | Canonical | A fact created by valid institutional procedure, such as a received report, ledger entry, posted notice, order, custody transfer, or case closure. It can be wrong, forged, stale, or contested. |
| Intention | Canonical | A durable commitment by an agent to pursue a goal, project, routine, obligation, or response until completion, failure, interruption, abandonment, or replacement. |
| Lead | Canonical projection | An actor-facing, source-bound projection of possible significance. A lead may suggest actions but does not imply truth, solvability, intended player involvement, completion, or reward. |
| Level of detail / LOD | Canonical | Explicit simulation fidelity for actors, places, regional processes, and history. Lower detail summarizes but must preserve necessary causal ancestry and promotability. |
| Memory | Canonical | Actor-specific retained information shaped by source, salience, recency, emotion, repetition, contradiction, relationship, role, and possible forgetting or distortion. |
| Need | Canonical | A pressure such as hunger, fatigue, or safety that influences candidate goals, routines, willingness, perception, and action selection. Needs are not cosmetic meters. |
| Norm | Canonical | A social or institutional rule such as obligation, prohibition, permission, privacy rule, status rule, evidence threshold, jurisdiction rule, or procedure rule. Norms can be violated. |
| Notice | Canonical | A physical or public information artifact with author or issuer, structured claims, source, location, readers, lifecycle, and stale risk. It is not a quest board item. |
| Observation | Canonical | Channel-specific perception or discovery by an actor or process. Observation is not interpretation and does not automatically transfer truth. |
| Ordinary agent | Canonical | Any simulated actor with normal world constraints: needs, beliefs, access, roles, relationships, intentions, and fallibility. A possessed actor is still ordinary. |
| Possession | Canonical | Non-diegetic binding of a human controller to an actor for input and embodied view. It changes control, not truth, knowledge, identity, memory, guilt, or privilege. |
| Project | Canonical | A persistent concern or long-horizon goal that can survive interruptions, such as repaying a debt, maintaining household food, preserving reputation, or recovering missing property. |
| Projection | Canonical | A derived, rebuildable read model from events and content versions, such as current state, belief store, actor notebook, ledger view, lead card, TUI view, or debug graph. |
| Proposition | Canonical | A structured claim that can be believed, reported, recorded, inferred, contradicted, or used in speech. A proposition is not automatically true. |
| Record | Canonical | A durable world artifact or institutional artifact containing structured claims, author, issuer, physical or digital location, source, readers, amendments, and lifecycle events. |
| Replay | Canonical | Rebuilding authoritative state and projections from event log, content versions, schemas, snapshots, and upcasters. Authoritative replay fails loudly on mismatch. |
| Routine | Canonical | A recurring, defeasible intention pattern such as sleeping, eating, working, office hours, patrol, or social visit. Routines have preconditions, duration, interruptions, and failure modes. |
| Salience | Canonical, context-bound | Observer-layer or memory-layer importance used to surface, summarize, retain, or inspect chains. Salience may highlight; it must not cause world events. |
| Scheduler | Canonical | The deterministic simulation-time system for wakeups, action durations, ordering, reservations, interruptions, no-human stepping, time acceleration, and replay. |
| Schema | Canonical | A versioned content or event contract with validation, stable IDs, source rules, references, compatibility, and migration expectations. A schema is not a script. |
| Speech act | Canonical | A typed communication action such as ask, answer, report, accuse, lie, gossip, promise, refuse, testify, recruit, threaten, or confess. It carries propositions, source beliefs, listeners, and validation. |
| Stale information | Canonical | Information that may once have been accurate but may no longer match ground truth. Staleness is tracked through source, event time, acquisition time, last verification, confidence, and contradiction. |
| Story sifting | Canonical projection | Observer/debug grouping or summarization of event chains after they occur. It may highlight patterns and contradictions but may not direct, spawn, pace, or repair events. |
| Trace | Canonical | A physical, mental, social, institutional, economic, environmental, or erasure residue left by an event that can later influence observation, belief, records, or action. |
| TUI | Canonical | The terminal-first playable client and acceptance surface. It consumes view models and submits commands; it is not a rules engine. |
| Validation gate | Canonical | A required proof that a feature satisfies action parity, actor knowledge, no-human operation, replay, TUI reachability, debug explanation, and no-script/no-LLM-authority boundaries. |
| Violation | Canonical | An action or state that breaks a norm. Violation, detection, suspicion, report, proof, sanction, and record are distinct. |
| View model | Canonical | A projection shaped for a consumer such as embodied TUI, debug UI, notebook, record view, or future graphical client. Embodied view models are actor-filtered. |
| Why-not UI | Canonical | Actor-filtered explanation of why an action is unavailable, risky, blocked, rejected, or differently classified, using state, belief, norm, resource, role, duration, or precondition reasons. |

## Restricted and forbidden-as-core terms

| Term | Status | Use instead / danger |
|---|---|---|
| Quest | Forbidden as core | Use incident, request, contract, obligation, notice, lead, record, suspicion, or procedure. Quest implies objective task truth, intended player involvement, completion, reward, and waiting world state. |
| Player character | Forbidden as core | Use actor for world entity and human controller for input source. Player character implies a privileged world identity. |
| NPC | Player-facing shorthand only | Use actor or agent in core docs, schemas, tests, and code. NPC frames the world around a player/non-player split. |
| Objective marker | Forbidden as core | Use actor-known lead marker, route belief, record claim, or affordance hint. Objective marker implies true destination or intended solution. |
| AI director | Forbidden as core | Use story sifter only for observer/debug projection. Director implies hidden pacing authority over causality. |
| Drama director | Forbidden as core | Same danger as AI director: pacing, excitement, or narrative need starts causing events. |
| Storyteller | Forbidden as core | Acceptable only in comparative discussion. Tracewake observes story after causality; it does not author events for story shape. |
| Script | Forbidden as core for ordinary play | Use action definition, routine, procedure, scenario seed, fixture setup, or boundary process. Script implies forced outcome sequence. |
| Scripted event | Forbidden as core | Use event generated by validated action or process. Fixture setup may create initial state, but normal play cannot rely on hidden scheduled outcomes. |
| Cutscene truth | Forbidden as core | Use event, observation, record, or actor-known summary. A presentation layer cannot declare world truth. |
| Omniscient NPC | Forbidden as core | Use actor with beliefs, observations, memory, sources, and inference. No ordinary actor reads ground truth. |
| Ground-truth dialogue | Forbidden as core | Use speech act backed by speaker belief, lie, speculation, question, record, or observation. Dialogue text cannot smuggle truth. |
| Hallucinated fact | Forbidden as core | Reject, repair, or classify as unsupported output. It is not a lie unless an actor intentionally performs a validated lie from a modeled motive/context. |
| Clue | Player-facing only | Use trace, observation, lead, record claim, rumor, or evidence. Clue implies it exists for the player; Tracewake artifacts exist because events left consequences. |
| Quest giver | Forbidden as core | Use requester, reporter, issuer, contract party, clerk, witness, authority, or notice author. Quest giver implies NPC exists to assign player tasks. |
| Reward | Restricted | Use payment, obligation, custody transfer, gift, debt settlement, sanction relief, or institutional compensation. Reward implies spawned payoff for completion. |
| Completion flag | Forbidden as core | Use world state, actor belief, record status, procedure status, obligation fulfillment, custody transfer, or institutional decision. Different layers can disagree. |
| Party member | Restricted | Use companion, recruited actor, contract party, co-traveler, escort, or follower with autonomy, needs, beliefs, and refusal conditions. |
| Global wanted meter | Forbidden as core | Use records, suspicion, testimony, proof thresholds, institutional beliefs, notices, and sanctions. Global meters imply omniscient law. |
| Map marker | Restricted | Use actor-known place belief, route lead, record location, visible landmark, or debug marker. Embodied markers must not point to hidden truth. |
| Spawn | Restricted | Use boundary event, birth, arrival, placement, creation event, fixture setup, or process output. Spawn often hides cause and provenance. |
| Solve | Player-facing only | Use recover, report, prove, close record, change belief, return property, satisfy obligation, or update procedure. A situation may be resolved differently by truth, actors, and institutions. |
