# Design Risk Register

## Status

This reference document is a living watchlist for unresolved, recurring, or easily-regressing design risks in Tracewake.

It is not a constitution, architecture contract, roadmap, issue tracker, or list of accepted tradeoffs. Settled doctrine belongs in the foundation and architecture layers. This register exists only to keep active failure modes visible while implementation, data authoring, validation, and playability pressure increase.

## Reference-layer decision

The reference layer should exist as a compact lookup and watchlist layer. It should stay small, stable, and low-duplication.

The design risk register should exist because Tracewake has several high-risk relapse modes that can appear correct locally while undermining the whole premise. The glossary should exist separately because terminology control is a different job from risk tracking.

Do not merge the risk register and glossary. Do not add more reference documents until a new reference need recurs across multiple foundation, architecture, execution, code, schema, test, and authoring discussions.

## Use and maintenance

Review this register:

- at every phase exit;
- before adding a new domain pack;
- before enabling any LLM surface beyond deterministic templates or mocks;
- before scaling population, geography, history, or LOD;
- before adding graphical presentation;
- after any validation, replay, epistemic-filtering, or TUI regression.

Status values:

- **Watch** — expected risk; monitor with tests, reviews, and metrics.
- **Active** — symptoms are present in docs, fixtures, code, schemas, UI, tests, or demos.
- **Escalated** — blocks phase exit or threatens a foundational premise.
- **Dormant** — no symptoms in current phase, but likely to recur under future pressure.
- **Retired** — removed only after sustained regression coverage and no plausible near-term recurrence.

A risk may be retired only when the team can name the tests, fixtures, reviews, metrics, or architectural boundaries that would catch its return.

## Risk watchlist

### R-01 — Protagonist gravity

- **Why it matters:** Tracewake fails if the world begins organizing around the human controller. The controlled actor must remain ordinary.
- **Symptoms:** Events cluster near the possessed actor without modeled cause; NPCs wait for the human; opportunities target the player; scheduler pauses unpossessed actors; UI summaries imply the controlled actor is the intended solver.
- **Mitigation / guardrails:** Run no-human simulations from the beginning. Compare embodied and no-human event distributions. Ban player-conditioned world logic in ordinary systems. Keep possession as controller metadata. Require NPC parity for world-affecting actions.
- **Evidence to watch:** Player-conditioned event rate; actions with no NPC equivalent; fixture data mentioning player identity; NPC idle loops while possessed actor is elsewhere; tests that only pass with a controller bound.
- **Status / review cadence:** Watch. Review at every runnable phase exit and whenever new TUI affordances or scheduler features are added.

### R-02 — Quest relapse

- **Why it matters:** Quest ontology imports objective truth, intended player involvement, completion flags, guaranteed rewards, waiting NPCs, and authored solution paths.
- **Symptoms:** Types, schema fields, UI labels, tests, or fixtures named `Quest`, `Objective`, `QuestTarget`, `Completion`, or `Reward`; notice boards behave like task menus; leads point to hidden truth; NPCs wait for acceptance.
- **Mitigation / guardrails:** Model incidents, requests, contracts, obligations, records, notices, rumors, leads, suspicions, and procedures instead. Treat leads as actor-known projections. Require source, uncertainty, and staleness for player-facing opportunities.
- **Evidence to watch:** Objective markers; completion flags; reward spawning; “accepted quest” conditions; tests asserting solved state instead of world, belief, record, and institutional state.
- **Status / review cadence:** Watch. Escalate immediately if quest-like schema enters core ontology.

### R-03 — LLM contamination

- **Why it matters:** No simulation fact may be born from prose. LLM output is nondeterministic, hard to validate, and prone to unsupported facts; it cannot be authoritative world state, agent cognition, institutional judgment, or proof.
- **Symptoms:** Dialogue creates facts; generated text changes state; NPCs know prompt-only information; prompt packets include hidden truth; live model output is required for tests; LLM summaries erase uncertainty.
- **Mitigation / guardrails:** Use structured speech acts, deterministic templates, deterministic mocks, prompt packets filtered by actor knowledge, output validation, unsupported-claim rejection, and LLM-disabled operation as the normal baseline.
- **Evidence to watch:** Test flakiness tied to live generation; unsupported claims accepted as lies; generated records without speech/report events; prompt packets containing true culprit, hidden item location, or other private beliefs.
- **Status / review cadence:** Watch now; Active when any live LLM surface is prototyped.

### R-04 — Simulation without game

- **Why it matters:** A perfect causal log is not enough. Tracewake must be playable, inspectable, and intervenable through a TUI; otherwise it becomes a simulator with no embodied game experience.
- **Symptoms:** Rich event logs but weak action menus; no why-not UI; no actor-known notebook; no readable leads; no satisfying waiting, searching, reporting, or conversation loops; play requires debug commands.
- **Mitigation / guardrails:** Keep every runnable phase TUI-playable or view-model-testable. Design affordance menus from actor-visible possibilities. Provide why-not explanations, actor-known notebooks, source-bound leads, and readable records early.
- **Evidence to watch:** Headless-only features; manual debug demos replacing embodied interaction; tests that ignore view models; player cannot form practical plans from actor-known UI.
- **Status / review cadence:** Watch. Review every time a subsystem claims “done.”

### R-05 — Scale before clarity

- **Why it matters:** Population, regional scope, procedural history, and LOD can hide shallow causality. A large world that cannot explain one missing item is hollow.
- **Symptoms:** More agents, settlements, routes, factions, or history before the first village works; low-LOD people become props; procedural generation precedes hand-authored causal fixtures; performance work replaces semantic clarity.
- **Mitigation / guardrails:** Prove the small village first. Expand only when replay, no-human simulation, actor knowledge, records, TUI, and debug explanations remain strong. Promote/demote LOD explicitly and preserve ancestry.
- **Evidence to watch:** Large fixtures with weak beliefs; uninspectable summary events; active leads whose causes were compacted away; metrics emphasizing population count over causal coverage.
- **Status / review cadence:** Watch. Escalate before any population, region, or procedural-generation expansion.

### R-06 — Agent stupidity or planner explosion

- **Why it matters:** Tracewake needs ordinary competence without opaque or unbounded planning. Agents must sleep, eat, work, store, search, ask, lie, refuse, report, and adapt under partial knowledge.
- **Symptoms:** Lifeless routines; actors freeze when unpossessed; every actor replans every tick; utility jitter; planner time dominates; plans mutate state directly; agents know truth for convenience; failed actions produce no recoverable behavior.
- **Mitigation / guardrails:** Use durable intentions, needs, projects, roles, relationships, HTN-style methods, bounded local planning, explicit planner budgets, fallback actions, and planner traces. Keep cognition symbolic and inspectable.
- **Evidence to watch:** Planner failures; stuck actors; repeated rejection storms; excessive planning nodes; actor choices with no belief or motive source; routines without failure modes.
- **Status / review cadence:** Watch. Active during routine/planner implementation.

### R-07 — Epistemic leakage

- **Why it matters:** Tracewake’s play depends on divergence between truth, belief, public claim, and institutional record. Leakage collapses the whole epistemic premise.
- **Symptoms:** Embodied view shows hidden culprit, true item location, lie labels, or other actors’ private beliefs; institutions read event-log truth; speech reveals unsupported facts; actor notebook inherits previous possession knowledge.
- **Mitigation / guardrails:** Require knowledge contexts on planner and view-model queries. Separate ground truth, subjective belief, and public/institutional records. Test multi-possession scenarios. Make actor-known projections the only source for embodied UI.
- **Evidence to watch:** View models built from raw state and then filtered manually; tests checking truth rather than holder/source/confidence; hidden event references in embodied prompt packets or notes.
- **Status / review cadence:** Watch. Escalate on any hidden truth in embodied mode.

### R-08 — Event-log and replay bloat

- **Why it matters:** Event sourcing is essential, but uncontrolled event volume or poor schema discipline can make replay, storage, migration, and inspection unusable.
- **Symptoms:** Every microscopic calculation becomes an event; replay slows before mechanics mature; snapshots erase active ancestry; event schemas churn without upcasters; old logs become unreadable; compaction hides causes of beliefs or records.
- **Mitigation / guardrails:** Commit causally meaningful events, not every calculation. Use summary events, snapshots, projection rebuild, event versioning, and upcasters. Preserve ancestry for active traces, beliefs, leads, records, suspicions, and procedures.
- **Evidence to watch:** Events per simulated day; replay time; projection rebuild time; old-fixture replay failures; missing causal references after compaction; active records pointing to summarized-away detail.
- **Status / review cadence:** Watch. Review at every replay/storage change.

### R-09 — Causal graph opacity

- **Why it matters:** The event log must answer why. A causal graph that technically exists but cannot explain action availability, wrong beliefs, traces, records, and consequences is dead weight.
- **Symptoms:** Events have IDs but no useful cause links; debug answers are prose guesses; beliefs lack provenance; wrong suspicion cannot be traced; random branches are unexplained; action rejection reasons are not recoverable.
- **Mitigation / guardrails:** Use typed causal links, validation reports, belief sources, trace origins, record provenance, meaningful random-draw labels, and debug queries shaped around “what caused this?” and “who knows what?”
- **Evidence to watch:** Events missing cause; debug “unknown” responses; orphan beliefs/records/traces; unqueryable validation reports; fixtures that cannot explain failure cases.
- **Status / review cadence:** Watch. Review whenever a new event family or projection is added.

### R-10 — TUI-first erosion

- **Why it matters:** The TUI is Tracewake’s first playable surface and early acceptance harness. If it becomes secondary, implementation will drift toward headless simulation, graphical ambition, or debug-only play.
- **Symptoms:** Mechanics ship without TUI/view-model tests; future graphical assumptions shape data; debug commands substitute for embodied actions; TUI duplicates rules; terminal UX cannot explain available actions or uncertainty.
- **Mitigation / guardrails:** Require semantic TUI action selection, actor-filtered view models, why-not explanations, transcript replay, and debug/embodied separation in every runnable phase. Keep future graphical clients behind view-model boundaries.
- **Evidence to watch:** Headless features marked accepted; no stable semantic action IDs; UI code enforcing world rules; acceptance demos that require debug mode.
- **Status / review cadence:** Watch. Review every feature acceptance claim.

### R-11 — Schema churn and authoring drift

- **Why it matters:** Tracewake depends on authored possibility space. If schemas drift, writers and programmers will smuggle in scripts, synonyms, hidden truth, and non-replayable facts.
- **Symptoms:** Data format chosen before validation needs are clear; inconsistent names for the same concept; records without author/provenance; beliefs without source; domain packs bypass action validation; fixture setup forces outcomes.
- **Mitigation / guardrails:** Keep logical contracts ahead of syntax. Validate stable IDs, references, source-backed beliefs, record artifacts, action registry parity, no-player fields, no-quest fields, no outcome chains, and content-version compatibility.
- **Evidence to watch:** Validation failures; schema migrations without fixtures; content examples containing `script`, `objective`, `true_culprit_for_institution`, or direct inventory mutation; author confusion around terms.
- **Status / review cadence:** Active during data-authoring design.

### R-12 — Validation theater

- **Why it matters:** Tests can make a bad simulation look safe if they assert happy paths, current truth, or demo behavior instead of doctrine-threatening failure modes.
- **Symptoms:** Manual demos replace no-human runs; tests check success only; institutions are tested against truth rather than institutional knowledge; LLM wording appears in golden tests; replay failures are tolerated; metrics are collected but ignored.
- **Mitigation / guardrails:** Require negative tests, property tests, no-human runs, replay/projection rebuild, epistemic leakage tests, TUI/view-model tests, and debug explanation assertions. Treat failure cases as deliverables.
- **Evidence to watch:** Green builds with no wrong-belief scenarios; no rejection/failure fixtures; absence of replay tests; tests that pass while embodied UI cannot play the feature.
- **Status / review cadence:** Watch. Escalate when a phase claims acceptance without failure evidence.

### R-13 — Genre lock-in

- **Why it matters:** The first domain is a restrained test domain, not the kernel identity. Genre assumptions in core ontology will block future domains and distort ordinary-life causality.
- **Symptoms:** Kernel types for magic, monsters, adventurers, taverns, feudal law, combat classes, or medieval-only currency; supernatural channels bypass belief; domain pack adds player-only verbs; fantasy threats become default proof.
- **Mitigation / guardrails:** Keep the kernel genre-agnostic. Put setting vocabulary, bodies, institutions, technologies, special information channels, and threats in domain packs that obey core action, belief, event, and validation rules.
- **Evidence to watch:** Core schemas with domain-specific nouns; first-proof data treated as universal; magic or combat bypassing ordinary perception, traces, norms, and records.
- **Status / review cadence:** Dormant now; review before any new domain pack or supernatural system.

### R-14 — Institutional omniscience

- **Why it matters:** Institutions are fallible social machines. If guards, clerks, ledgers, or norms know truth automatically, wrong suspicion, stale records, and social consequence collapse.
- **Symptoms:** Guards know crimes instantly; records appear because violations occurred; proof thresholds read true culprit; sanctions occur without detection/procedure; global wanted meter; notice auto-updates from truth.
- **Mitigation / guardrails:** Keep violation, detection, suspicion, report, record, proof, and sanction separate. Records require author, artifact, claims, source, readers, and lifecycle events. Institutions act from institutional knowledge, resources, roles, bias, and procedure.
- **Evidence to watch:** Institutional belief without source; record without reporter/receiver/writer; case opened by ground truth; suspicion formula using actual culprit.
- **Status / review cadence:** Watch. Active during institution implementation.

### R-15 — Story-sifting becoming direction

- **Why it matters:** Story sifting is allowed only as observer/debug interpretation after events. If it starts selecting culprits, pacing incidents, creating clues, or altering probabilities, it becomes a hidden director.
- **Symptoms:** Sifter spawns events; changes route risk because play is quiet; promotes threats for drama; delays discovery; creates clues to make a mystery solvable; rewrites summaries into authored arcs.
- **Mitigation / guardrails:** Make story-sifter outputs projections only. Forbid mutation authority. Keep summaries actor-filtered in embodied contexts and truth-capable only in debug. Test that removing the sifter leaves simulation outcomes unchanged.
- **Evidence to watch:** Sifter output consumed by scheduler, planner, event resolver, or content generator as authority; salience scores affecting probability; “pacing” fields in world logic.
- **Status / review cadence:** Watch. Review before adding observer summaries beyond debug recap.

### R-16 — Debug/forensic leakage into embodied play

- **Why it matters:** Tracewake needs omniscient debugging, but debug truth must never become actor knowledge or embodied UI truth.
- **Symptoms:** Debug view visually resembles embodied view; debug notes satisfy action preconditions; actor notebooks include hidden truth; possession history becomes world fact; debug injection is used in ordinary play; forensic summaries appear as actor-known summaries.
- **Mitigation / guardrails:** Keep debug view models visibly non-diegetic. Separate human/debug notes from actor notebooks. Route debug injections through explicit debug/test events. Test that embodied views and actions cannot consume debug-only projections.
- **Evidence to watch:** Shared projection types without mode tags; debug fields in embodied prompt packets; user-facing notebooks sourced from causal graph truth; tests that require debug commands to complete normal flows.
- **Status / review cadence:** Watch. Escalate on any debug truth in embodied mode.

### R-17 — Action-pipeline bypass

- **Why it matters:** The shared action pipeline is the enforcement point for parity, causality, knowledge, norms, traces, scheduling, and event commit.
- **Symptoms:** UI mutates state; data scripts move items; domain pack writes events directly; planner updates components; language surface commits facts; storage adapter becomes domain model; debug command indistinguishable from world action.
- **Mitigation / guardrails:** Require every world-affecting change to pass through command/proposal, validation, scheduling/resolution, event commit, projection rebuild, and debug explanation unless it is explicit test/debug metadata.
- **Evidence to watch:** Direct component writes outside event application; state changes with no validation report; authored fixtures with timed outcome sequences; boundary code owning domain rules.
- **Status / review cadence:** Watch. Escalate immediately on ordinary-play bypass.

### R-18 — LOD ancestry loss

- **Why it matters:** LOD is necessary for scale, but summaries that erase active causes destroy Tracewake’s forensic and epistemic value.
- **Symptoms:** Low-LOD actors become props; summary events hide origins of beliefs, records, leads, or suspicions; demotion erases traces; promotion invents missing ancestry; compaction makes replay explanation impossible.
- **Mitigation / guardrails:** Use explicit promotion/demotion metadata or events, summary events with retained causal facts, promotable low-LOD identities, and compaction blocks for active traces, records, suspicions, and leads.
- **Evidence to watch:** Active record points to missing event; promoted actor lacks home/role/beliefs/projects; debug cannot explain what detail was retained or lost; no-human regional run diverges under replay.
- **Status / review cadence:** Dormant until LOD work; review before any summary or compaction feature.
