# Spec 0005 — Phase 3A: Needs, Routines, and No-Human Day

**Status:** Draft implementation specification  
**Repository:** `joeloverbeck/tracewake`  
**Target commit analyzed:** `8fa8d1b473be848a879457d9a5dd06a2c86e24b3`  
**Filename:** `0005_PHASE_3A_NEEDS_ROUTINES_AND_NO_HUMAN_DAY_IMPLEMENTATION_SPEC.md`  
**Phase:** Phase 3A narrow ordinary-life substrate

North star:

> Boring life, ruthlessly replayable.

Phase 3A is not the “smart NPC” phase. It is not the theft story phase. It is not the institutions phase. It is the first executable ordinary-life substrate: actors have bounded needs, form durable intentions from those needs and routine duties, use authored routine methods and bounded local planning to submit ordinary action proposals through the existing action pipeline, can live a boring no-human day, and can explain failures aggressively.

---

## 1. Evidence ledger

Target commit analyzed: `8fa8d1b473be848a879457d9a5dd06a2c86e24b3` (equal to `main` HEAD at authoring; freshness is user-supplied, not independently re-verified). This spec was authored by reading the repository at that commit — the full Rust workspace (`crates/tracewake-{core,content,tui}`), the layered doc pack (`docs/0-foundation` through `docs/4-specs`), and the existing fixtures. The per-file fetch manifest and fetch-tool mechanics from the authoring pass have been omitted as non-enduring provenance, consistent with `docs/4-specs/SPEC_LEDGER.md` noting that exact fetch ledgers are not product doctrine.


External research consulted for design sharpening, not authority override:

- Meneguzzi and Luck / related BDI-planning survey lead: `https://www.cambridge.org/core/journals/knowledge-engineering-review/article/planning-in-bdi-agents-a-survey-of-the-integration-of-planning-algorithms-and-agent-reasoning/66B76DBDC23B31C14D378C020FF37831`
- Georgievski and Aiello, “An Overview of Hierarchical Task Network Planning”: `https://arxiv.org/abs/1403.7426`
- Jeff Orkin, “Three States and a Plan: The A.I. of F.E.A.R.”: `https://www.gamedevs.org/uploads/three-states-plan-ai-of-fear.pdf`
- Explainable agent / explainable NPC research leads, including EXAG and explainable goal-driven agents literature.

---

## 2. Executive summary

Spec 0005 must implement the narrow Phase 3A ordinary-life slice. The success condition is:

> Agents can live a boring ordinary day without the human, and when they fail, the engine can explain exactly why.

The core causal chain for this slice is:

1. Actor beliefs, embodied state, need pressures, duties, and routine windows produce candidate goals.
2. The actor selects or continues a durable intention.
3. The selected intention chooses an HTN-like routine method.
4. The routine method proposes concrete ordinary actions.
5. Concrete sequencing may use bounded local planning, but only from actor-available knowledge.
6. Every concrete action enters the shared action/proposal/pipeline machinery.
7. Event log, replay, projections, TUI, debug panels, and diagnostics expose what happened and why.

The first-slice needs are exactly:

- `hunger`
- `fatigue`
- `safety`

The required routine families are:

- `MorningWake`
- `EatMeal`
- `GoToWork`
- `WorkBlock`
- `ReturnHome`
- `SleepNight`
- `FindFood`
- `ContinueCurrentIntention`
- `Wait` / `IdleWithReason`

The required new ordinary actions are:

- `sleep`
- `eat`
- `begin_work` and/or `work_block`
- `continue_routine`
- extended `wait`

The required canonical no-human fixture is:

- `no_human_day_001`

The required canonical interruption proof is:

> Food unavailable -> actor replans or fails with a typed diagnostic.

The required diagnostic posture is deliberately harsh. A no-human run with silent actor freezing, repeated unreasoned idle loops, hidden truth planning, schedule teleportation, or no replayable explanation must fail.

---

## 3. Doctrine alignment

Phase 3A must obey the already-established Tracewake doctrine.

### 3.1 Causality first

All meaningful state change must be eventful or replay-reconstructible with explicit ancestry. Needs, routine starts, routine step transitions, interruptions, sleep starts/completions, food consumption, work blocks, planner traces, and stuck diagnostics must be visible in the event/replay/projection story. No Phase 3A feature may mutate actor state behind the event log merely because “time passed.”

### 3.2 No protagonist gravity

No-human day events must not reference a player identity, controller identity, player objective, quest state, player memory, or any player-conditioned hidden flag. Human possession may change the controlled viewpoint and command origin, but it must not reset needs, intentions, routines, or future commitments.

### 3.3 No hidden truth planning

Autonomous actors may reason from actor-known facts, current embodied perception, remembered/believed facts, authored routine knowledge, and public domain knowledge that the fixture grants them. They must not use ground-truth container contents, hidden item locations, hidden culprit facts, future schedule outcomes, or objective story needs to choose intentions or concrete actions. The action validator may check authoritative physical truth to accept or reject a proposal; the planner may not inspect that truth as a decision oracle.

### 3.4 Ordinary actions are not player verbs

Every Phase 3A action must be available to autonomous actors and human-possessed actors through the same semantic action/proposal/pipeline structure. `sleep`, `eat`, `work_block`, and `continue_routine` must not be TUI-only commands, debug-only state changes, or player-only affordances.

### 3.5 Needs are pressures, not puppet strings

A high hunger value may increase pressure to eat and justify interrupting work. It must not force telepathic knowledge of food, bypass access, bypass movement, bypass locked containers, override beliefs, ignore safety, or direct mutate hunger downward without a food/service event.

### 3.6 Routines are causal machinery, not scripts

A routine may author an ordinary procedure. It must not teleport an actor, move items directly because a story beat requires it, force success regardless of blockers, or continue silently after a failed precondition. Routines are defeasible durable intentions that can be interrupted, suspended, resumed, completed, failed, or abandoned.

### 3.7 Debug visibility is allowed; debug authority is not

Debug panels may reveal full needs, routine state, planner traces, rejected candidates, and stuck diagnostics. Debug views must not mutate actor knowledge, add observations, repair failures, or become simulation authority.

### 3.8 LLMs remain outside authority

Phase 3A must have no LLM agent brains, no LLM action selection, no LLM dialogue, and no LLM-authored hidden facts. Research may shape the planner vocabulary, but Tracewake-native symbolic, deterministic, inspectable machinery wins.

### 3.9 Binding invariants and architecture constraints

The doctrine alignment above is governed by these constitutional invariants (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`) and architecture contracts. Every Phase 3A deliverable must satisfy them at least as strictly as the constitution requires:

- **INV-004** (the authoritative world ignores human existence) and **INV-006** (possession transfers no world knowledge) — the no-human day runner (§10.3) and the possession-does-not-reset-intention guarantee (§8.4).
- **INV-008** (UI assistance is not authority) — debug surfaces inspect but never mutate state, add observations, or decide legality (§3.7, §14.2).
- **INV-009 / INV-010 / INV-011 / INV-013** (meaningful changes require events, every event needs a cause model, current-state-only simulation is forbidden, meaningful events leave traces) — eventful or replay-reconstructible needs, routine transitions, interruptions, and diagnostics (§3.1, §8, §15).
- **INV-017 / INV-018 / INV-019** (seedable/auditable randomness, deterministic replay, snapshots/compaction preserve ancestry) — deterministic decision ordering and replay of Phase 3A state (§10.1, §15).
- **INV-024** (no telepathy) — no hidden-truth planning and no embodied-view leakage of other actors' or ground-truth facts (§3.3, §14.1).
- **INV-032 / INV-033 / INV-034 / INV-035 / INV-036 / INV-037** (symbolic inspectable agents, BDI separation, durable intentions, defeasible routines, HTN methods as procedures not scripts, bounded local planning for concrete means) — the needs/intentions/routine/planner model (§8.4, §8.5, §11).
- **INV-039** (needs are pressures, not puppet strings), **INV-045** (ordinary survival is causal; no fake meter refills), and **INV-046** (V1 economy is simplified but not fake) — need pressures and eventful food/sleep/work effects, with work output kept a non-economic marker pending later economy work (§3.5, §8.1, §9, §9.8, §21).
- **INV-041** (agent decisions need debug traces) — mandatory planner/decision traces for salient autonomous choices (§8.7).
- **INV-042** (no LLM agent brains) — symbolic, deterministic machinery only (§3.8).

Governing architecture contracts: `docs/1-architecture/01_SYSTEM_AUTHORITY_RUST_WORKSPACE_AND_BOUNDARIES.md` (kernel authority and dependency direction), `docs/1-architecture/02_ACTION_AFFORDANCE_SCHEDULING_AND_PROPOSAL_PIPELINE.md` (the shared proposal pipeline every Phase 3A action must use), and `docs/1-architecture/07_AGENT_COGNITION_ROUTINES_PLANNING_AND_AGENT_LOD.md` (agent cognition, routines, and planning).

---

## 4. Current codebase assessment

This assessment is based only on exact-commit files listed in the evidence ledger.

### 4.1 What is already present

The workspace is split into `tracewake-core`, `tracewake-content`, and `tracewake-tui`. The existing dependency direction and crate boundaries already support a core-authoritative simulation with content loading and a TUI surface.

The core currently has a physical state substrate for actors, places, doors, containers, items, controller bindings, locations, custody/value placeholders, checksums, and deterministic ordered collections. It supports actor bodies at places, items in places/containers/actors, open/closed and locked containers/doors, local visibility/reachability, and carry/capacity constraints.

The event substrate already includes typed event envelopes, event streams, ordering keys, schema/version fields, event application, event logging, replay/rebuild support, checksums, and diagnostic/controller/replay-debug streams. Existing event kinds cover controller binding, movement, open/close, take/place, wait/time advancement, no-human advance markers, and Phase 2A epistemic records.

The scheduler already has deterministic ordering concepts: schedule phase, scheduler source, proposal sequence, ordering keys, and a no-human advance process. The current no-human process can mark no-human advancement, sort scheduled proposals, run proposals through the shared pipeline, and advance ticks. That is an excellent skeleton, but it is not yet an autonomous ordinary-life day.

The action system already has a shared proposal model with origins including `Human`, `Test`, `Scheduler`, `Agent`, and `Debug`. The registry currently exposes implemented effects for movement, open, close, take, place, wait, check-container, and query-only probes. The pipeline already runs validation/decision/application stages, applies accepted events, records action rejection, and has placeholders for knowledge/perception, social norms, cost/duration, and reservations. These placeholders must become real enough for Phase 3A where needed; they must not remain misleading labels for unimplemented ordinary-life features.

The Phase 2A epistemic substrate already has propositions, observations, beliefs, contradictions, knowledge context, projections, and notebooks/debug epistemic views. Existing check-container and take/place hooks can create observations and contradiction-style absence evidence. This is the right foundation for “actor believed food was in the pantry; checked; discovered it was absent; replanned.”

The TUI already has embodied view generation, semantic action exposure, command handling, transcript tests, debug panels, notebook/debug epistemics, binding, waiting, and action selection. The TUI pattern must be extended rather than bypassed.

The content crate already has fixture schema/load/serialization/validation, deterministic fixture tests, forbidden content checks, and golden fixtures such as `strongbox_001`, `expectation_contradiction_001`, `possession_parity_001`, `no_human_advance_001`, and `no_human_epistemic_check_001`. Validation already rejects some shortcut/quest/player/hidden-truth fields.

### 4.2 What is not yet present

The exact-commit implementation does not yet provide first-class ordinary-life cognition. Inspected core/content/TUI files do not contain a real `NeedState`, routine template/instance/intention stack, planner or decision trace substrate, autonomous candidate-goal generation from needs/routines, sleep/eat/work action families, full-day no-human ordinary-life runner, or aggressive stuck-actor diagnostics.

The current no-human fixture is a marker/advance proof, not a day. It proves that no-human advancement can occur without a controller, not that actors can wake, eat, travel, work, return, sleep, replan, and explain failures.

The current content schema models actors, places, doors, containers, items, affordances, and initial beliefs. It does not yet validate homes, sleep places, food supplies/services, workplaces, initial needs, routine assignments, routine steps, routine failure modes, no-human day manifests, or anti-teleport routine constraints.

The current TUI/debug view models do not expose needs, intention/routine state, planner traces, candidate goals, rejected methods, stuck actor diagnostics, no-human day summaries, or sleep/eat/work/continue affordances.

The current replay/checksum/projection path must be extended so Phase 3A events and reconstructed state are first-class, not debug-only side effects.

### 4.3 Implementation posture

Phase 3A must extend existing seams rather than replace them:

- Keep ordinary actions in the shared action registry/pipeline.
- Keep autonomous actions as proposals, not direct mutations.
- Keep no-human execution deterministic and replayable.
- Keep actor-visible action availability in view models.
- Keep debug inspection non-diegetic and non-authoritative.
- Keep content validation as an architectural gate, not a late convenience.

---

## 5. Phase 3A purpose and boundaries

### 5.1 Purpose

Phase 3A creates the smallest honest substrate for ordinary autonomous life:

- Bounded needs change over time and action.
- Needs and routine windows generate candidate goals.
- Actors commit to durable intentions.
- Intentions select routine methods.
- Routine methods propose ordinary actions.
- Actions are validated through the same pipeline used by the possessed actor.
- No-human runs exercise a full day without player presence.
- Failure is typed, replayable, and explainable.

The phase is successful when the engine can run a deliberately boring day and then answer, in debug:

- Why did Mara try to eat?
- What did Mara believe about food?
- What action did she attempt?
- Why did it fail?
- What fallback was considered?
- Why did she continue, wait, replan, abandon, or fail?
- Did any hidden truth or player state affect the result?
- Can replay rebuild the same state, traces, and diagnostics?

### 5.2 Hard boundaries

This spec is only Phase 3A. It must not expand into:

- full Phase 3 completion;
- full missing-property autonomous story;
- Phase 4 reports, records, institutions, legal authority, or wrong suspicion workflows;
- mature household institutions beyond minimal domestic access/sleep/food tags;
- full economy, wages, prices, markets, debt law, or payment ledgers;
- speech/dialogue/report/gossip systems;
- notices as a product feature;
- regional travel, roads, bounties, companions, threats, combat, injury, or graphical client work;
- LLM brains or LLM dialogue;
- large population simulation;
- procedural town generation;
- quest/objective ontology;
- dramatic director logic.

A placeholder for a later system may be introduced only if it is inert, validated, explicitly non-authoritative, and necessary to prevent shortcut design. For example, a workplace may have `work_output_tag: service_completed_placeholder`; it must not become wages, debt law, institutional reporting, or story reward logic.

---

## 6. Entry requirements and assumptions

Phase 3A assumes the following exact-commit substrate remains available:

- physical actor/place/door/container/item state;
- deterministic IDs and ordered state collections;
- event envelope/log/apply/replay/checksum machinery;
- shared semantic action registry and proposal pipeline;
- existing movement, open/close, take/place, wait, check-container actions;
- proposal origins including agent/scheduler;
- deterministic scheduler ordering and no-human process skeleton;
- Phase 2A epistemic propositions, observations, beliefs, contradictions, knowledge context, notebook/debug views;
- content fixture schema/load/validation tests;
- TUI embodied command loop, semantic action rendering, debug panels, and transcript tests.

The implementation may introduce a broader simulation state wrapper if needed, but it must not erase the existing physical/event/pipeline separation. In particular, do not cram cognition into ad hoc fields on `ActorBody` merely because it is convenient. The preferred direction is an authoritative agent/cognition state keyed by actor, threaded through scheduler/pipeline/replay/projections alongside physical and epistemic state.

---

## 7. Deliverables

The Phase 3A implementation must deliver the following cohesive behavior.

### 7.1 Core simulation deliverables

- Authoritative need state for `hunger`, `fatigue`, and `safety`.
- Need deltas from time, sleep, eating, work, waiting, and relevant routine/action effects.
- Candidate goal generation from needs, routine windows, duties/place assignments, and current intention state.
- Durable current intention state per actor.
- Routine templates and routine executions.
- Routine step transitions, failures, interruptions, fallback/replanning behavior.
- Planner/decision trace records for salient autonomous decisions.
- Aggressive stuck-actor detection and typed diagnostics.
- Sleep, eat, work, continue-routine, and extended wait actions through the shared pipeline.
- No-human day runner exercising the canonical day fixture.

### 7.2 Content deliverables

- Schema support for initial needs.
- Schema support for homes, sleep places, food stores/services, workplaces/work stations.
- Schema support for routine templates or routine assignments.
- Schema support for day windows/routine windows.
- Validation for routine preconditions, failure modes, interruption triggers, and no-teleport constraints.
- Canonical fixtures including `no_human_day_001` and smaller golden fixtures listed later.

### 7.3 TUI and debug deliverables

- Embodied needs/status summary for the possessed actor.
- Embodied current intention/routine summary in actor-known terms.
- Embodied affordances for sleep/eat/work/wait/continue.
- Why-not explanations for unavailable ordinary-life actions.
- Debug views for all actors' needs, intentions, routines, planner traces, candidate goals, rejected reasons, and stuck diagnostics.
- No-human day summary/debug report.
- Replay/projection comparison support for new Phase 3A state.

### 7.4 Test deliverables

- Unit, integration, golden fixture, TUI/view-model, replay, validation, and regression/property tests.
- No-human day acceptance gate.
- Explicit no-hidden-truth planning test.
- Explicit no-teleport routine test.
- Explicit possession-does-not-reset-intention test.
- Explicit repeated-idle/stuck-actor failure tests.

---

## 8. Data and model requirements

The exact Rust type names are implementation choices, but the following semantic entities are mandatory. The code must expose equivalent concepts in tests, event payloads, projections, or debug reports.

### 8.1 Need state

Each actor with ordinary-life autonomy must have a bounded deterministic need state for:

- hunger;
- fatigue;
- safety.

Use an integer scale. The recommended canonical scale is `0..1000`, where higher means more pressure. Bands:

| Band | Inclusive range | Meaning |
|---|---:|---|
| comfortable | 0–249 | no immediate pressure |
| rising | 250–499 | can influence routine choices |
| urgent | 500–749 | can interrupt weak intentions |
| severe | 750–1000 | must be considered before ordinary duties unless safety says otherwise |

The implementation may use `0..100` only if tests prove identical bounded/banded semantics. Do not use floating point needs.

Need values must be clamped. Tests must prove underflow/overflow cannot occur.

Need state must include enough ancestry for replay/debug to answer why a value changed. Action-caused changes must be eventful. Passive time changes may be replay-reconstructible from deterministic day-window deltas, but threshold crossings and any need change that contributes to a decision must have explicit event ancestry or trace ancestry.

Required need effects:

- Hunger rises with awake time and work.
- Hunger falls only through successful food consumption or food-service use.
- Fatigue rises with awake time and work.
- Fatigue falls through scheduled sleep/rest completion over time, not instant refill.
- Safety pressure rises in actor-known unsafe/private/blocked/dark/unknown situations where Phase 3A models those tags.
- Safety pressure falls when the actor reaches a safe/known/allowed place or resolves the immediate blocker.

Safety is minimal. It is not combat, injury, guard behavior, fear psychology, or a full norm system. It is a pressure to avoid or leave obviously unsafe/inaccessible/unknown situations using actor-known information.

### 8.2 Need pressure

A `NeedPressure` is a derived evaluation at a tick, not a separate hidden motive. It must include:

- actor;
- need kind;
- value and band;
- threshold crossing if any;
- source ancestry: tick delta, action, routine effect, or fixture initial value;
- whether it can interrupt the current intention;
- actor-known explanation string or projection fragment;
- debug-only exact details.

Needs must not directly choose actions. They create pressure and candidate goals.

### 8.3 Candidate goal

A `CandidateGoal` represents an inspectable option before commitment. Required fields/semantics:

- actor;
- tick/window;
- source: need pressure, routine duty, current intention continuation, safety interruption, fixture routine assignment, or fallback;
- goal kind: eat, find food, sleep, work, go home, continue current intention, idle with reason, leave unsafe place, or equivalent;
- priority band/reason;
- belief/perception inputs used;
- applicability result;
- rejection reason if not selected;
- selected routine/method if chosen;
- trace ID.

Candidate goals must be deterministic. Tie-breaking must be explicit and stable. The recommended default order is:

1. severe safety pressure;
2. severe hunger;
3. severe fatigue/sleep need;
4. active durable intention continuation unless interrupted;
5. urgent hunger/fatigue;
6. routine window duties such as work;
7. return home/sleep-night window;
8. idle/wait with reason;
9. stable actor ID / goal ID tie-break.

The default selection rule must prefer the active intention over mild need pressure. This prevents jitter. Severe/urgent thresholds and explicit interruption points are how an actor changes course.

### 8.4 Durable current intention

Every autonomous actor must have zero or one current intention, plus optionally suspended historical intentions if needed for resumption. An intention must carry:

- actor;
- source/trigger;
- selected goal;
- selected routine/method;
- current step;
- commitment/durability level;
- start tick;
- last-progress tick;
- status: active, suspended, completed, failed, abandoned, or interrupted;
- failure/interruption reason when applicable;
- trace ID or ancestry.

Intentions must survive:

- time advancement;
- no-human execution;
- TUI wait;
- possession binding/unbinding;
- controller transfer;
- replay/rebuild.

Possession must not copy, reset, reinitialize, or complete an intention. A possessed actor must be able to continue the current intention through the same machinery as an autonomous actor.

### 8.5 Routine templates and routine executions

A `RoutineTemplate` is an authored symbolic method. A `RoutineExecution` or `RoutineInstance` is the actor-specific runtime execution of that method.

A routine template must define:

- routine family/kind;
- applicability conditions;
- preconditions;
- steps;
- expected duration or duration band;
- interruption points;
- failure modes;
- fallback/replanning rules;
- debug trace labels;
- whether it may reserve exclusive resources such as a bed or workstation;
- action proposal strategy for each concrete step.

A routine execution must define:

- actor;
- template/method;
- current step;
- step status;
- start tick;
- last-progress tick;
- expected next progress tick or deadline;
- concrete action ancestry;
- any reserved resource;
- fallback attempts already made;
- failure/interruption state;
- trace ID.

No routine step may directly move an actor, directly move an item, directly alter hunger/fatigue, or directly complete work. Routine steps submit ordinary action proposals or wait with a typed reason.

### 8.6 Routine step semantics

The minimal routine step vocabulary must be small and concrete. Examples:

- select known place;
- move toward place;
- open/check known container;
- consume accessible food;
- start scheduled sleep;
- complete scheduled sleep when due;
- start work block;
- complete work block when due;
- wait until window/resource/event;
- continue current step;
- fallback to `FindFood`;
- fail with typed diagnostic.

A routine step must not be a story beat such as “be absent so theft can happen.” It must be an ordinary procedure with physical/action ancestry.

### 8.7 Planner/decision trace

Planner traces are mandatory. A trace must be created for every salient autonomous decision:

- initial candidate-goal evaluation;
- intention adoption;
- intention continuation vs switch;
- routine method selection;
- action proposal generation;
- failed precondition;
- fallback/replan;
- abandonment/failure;
- stuck diagnostic.

Trace records may be event payloads, deterministic debug projections, or both. They must be replay-safe enough for acceptance tests. They must include:

- trace ID;
- actor;
- tick/window;
- active needs and bands;
- current intention before decision;
- candidate goals considered;
- selected goal and selected method;
- rejected goals/methods/actions and reasons;
- beliefs/perceptions/known places used;
- action proposal attempted;
- action validation result;
- fallback considered;
- hidden-truth audit result;
- outcome: continued, switched, waited, replanned, failed, or completed.

A trace must never mutate actor knowledge. Debugging the trace must not create observations.

### 8.8 Stuck actor diagnostic

A stuck actor diagnostic is not optional. Phase 3A must be aggressive.

A diagnostic must be emitted or surfaced when any of the following occur:

- an actor has an active intention but no progress past its expected progress window;
- an actor repeats idle/wait without a typed reason;
- a routine step fails and no fallback is recorded;
- a planner repeatedly selects an unsupported action;
- a needed resource is missing or inaccessible;
- a planner budget is exhausted;
- a fixture lacks required home/sleep/food/work/routine data;
- a no-human run ends with an actor in an unexplained active or idle state.

Diagnostic fields:

- actor;
- tick/window;
- active need/goal/intention;
- routine/method/step;
- attempted action/proposal;
- blocker category;
- concrete blocker;
- actor-known explanation;
- debug-only details;
- retry/abandon/fallback outcome;
- resulting status: idle, waiting, replanning, failed, suspended, or completed.

Blocker categories are exactly:

- physical;
- access;
- knowledge;
- resource;
- social/norm placeholder;
- scheduling/reservation;
- unsupported-action;
- planner-budget-exhausted;
- fixture-authoring-error.

A no-human run with repeated idle loops or no progress must fail tests unless the fixture explicitly expects the typed failure and the actor enters a stable explained state.

---

## 9. Action pipeline requirements

Phase 3A ordinary-life actions must extend the existing shared action system. They must not bypass it.

### 9.1 Required action IDs/semantics

The action registry must support semantic actions equivalent to:

- `sleep`;
- `eat`;
- `begin_work` and/or `work_block`;
- `continue_routine`;
- extended `wait`.

Exact naming can vary only if action IDs remain stable, tested, and exposed through view models/debug. The canonical fixture/test names must use the IDs above unless there is a strong existing naming constraint.

### 9.2 Proposal generation

Each required action must support proposal generation from:

- human command/action menu;
- autonomous actor decision/routine;
- scheduler continuation where duration/action completion requires it;
- tests.

Debug may create proposals only through explicit debug origin and must not be used to satisfy ordinary acceptance.

### 9.3 Validation and outcome distinction

The pipeline must distinguish:

- rejected proposal: the action cannot start because preconditions fail now;
- started action failure: the action started but failed/interrupted because conditions changed, resource disappeared, duration ended in failure, or routine cancellation occurred;
- completed action: the action completed with eventful/replayable effects.

Examples:

- Trying to eat from a locked/inaccessible pantry is a rejected proposal.
- Starting sleep and being interrupted before completion is a started action followed by interruption/failure.
- Starting work and failing because the workplace became inaccessible is a started action failure.
- Continuing a routine with no current intention is a rejected proposal with why-not.
- Continuing a routine whose next step is unsupported is a stuck diagnostic plus rejected/failed transition.

### 9.4 Duration and scheduling

Sleep and work must be duration-based. They must not complete instantly.

A duration action must produce:

- start event;
- expected completion tick or scheduled continuation;
- resource/body reservation where needed;
- completion/interruption/failure event;
- need deltas caused by elapsed duration;
- replay rebuild support.

The scheduler may reuse its deterministic ordering keys and no-human/deferred phases. It must not skip committed events during time advancement. If multiple scheduled completions occur at the same tick, ordering must be stable and traceable.

### 9.5 Reservation/conflict

At minimum, the engine must prevent one actor body from performing overlapping body-exclusive actions. An actor cannot simultaneously sleep, work, move, and eat.

If beds or workstations are modeled as exclusive resources, reservation conflicts must be typed as `scheduling/reservation` blockers. If exclusivity is not implemented for a resource, the debug trace must say that it is not enforced in Phase 3A; it must not pretend a reservation was checked.

### 9.6 Sleep

Sleep must be scheduled/duration-based, not an instant meter refill.

Required validation:

- actor exists and is enabled;
- actor has an accessible sleep place or bed known/visible enough to use;
- actor is at the required place or can route there through ordinary movement;
- bed/sleep place is not blocked by implemented occupancy/reservation rules;
- actor is not already in a body-exclusive action.

Required events/effects:

- sleep intention/routine step ancestry;
- sleep start event;
- scheduled sleep completion or interruption event;
- fatigue recovery over duration;
- possible hunger increase over duration;
- safety adjustment only from modeled safe/unsafe sleep context;
- replay rebuild of sleeping state and need values;
- TUI embodied affordance and why-not.

No Phase 3A sleep implementation may instantly set fatigue to comfortable.

### 9.7 Eat

Eating must consume world-modeled food or use a world-modeled food service. It must not be an abstract “hunger refill” command.

Required modeling:

- a simple food item/stack with servings, or a simple food-service entity with finite served-use ancestry;
- location/access/permission checks;
- actor-known/visible/believed food source selection;
- eventful consumption or service use;
- hunger reduction;
- possible item/serving decrement or service-use record;
- failure if no accessible food exists;
- fallback to `FindFood`, wait, or typed failure depending on available systems.

The recommended first implementation is a finite `food_stack`/`food_supply` content entity with integer servings and deterministic hunger reduction per serving. A food service may be a simple fixture entity that consumes one serving from a backing store or records service use without introducing money. Do not implement cooking, perishability, calories, nutrition, prices, trade, or full shop economy.

Eating must not use hidden true food locations. The planner may choose a known/believed food location; the action validator may reveal that the food is absent or inaccessible by rejection/failure and observation ancestry.

### 9.8 Work

Work must be causal but not a full economy.

Required validation:

- actor exists and is enabled;
- actor has a workplace/workstation assignment or routine method;
- actor is at the workplace or can route there through ordinary movement;
- workplace/access conditions permit work;
- actor is not blocked by hunger/fatigue/severe safety according to Phase 3A thresholds;
- actor is not already in a body-exclusive action.

Required events/effects:

- work intention/routine step ancestry;
- work block start event;
- scheduled completion or failure event;
- time cost;
- fatigue/hunger impact;
- actor absence from home through ordinary movement ancestry;
- simple output such as `work_block_completed`, `service_completed_placeholder`, or `availability_fact`.

Do not create money from nowhere. If payment is included despite this warning, it must be eventful and custody-aware. The preferred Phase 3A direction is no wages/payment, only a non-economic work-completion marker.

### 9.9 Continue routine

`continue_routine` must submit the next ordinary proposal for the actor's active intention or produce a why-not/diagnostic explaining why it cannot.

It must not be a debug cheat. It must not jump steps. It must not mutate the intention directly. It must not ignore action validation.

Required behavior:

- If no current intention exists: reject with `NoCurrentIntention` or equivalent why-not.
- If current intention is completed/failed: reject or surface summary.
- If next step can produce an ordinary action: submit that proposal.
- If next step needs routing/planning: invoke bounded planner and trace it.
- If next step is blocked: emit routine failure/interruption/replan or stuck diagnostic.
- If actor is possessed: use the same state and routine execution; do not reset or copy.

### 9.10 Extended wait

The existing `wait` action must be extended so time advancement interacts with needs/routines rather than merely emitting a trivial wait event.

Required behavior:

- wait has a reason when used autonomously;
- wait applies/reconstructs passive need deltas;
- wait can trigger threshold crossings and candidate-goal reevaluation;
- wait/continue time control stops for actor-perceived salient interruptions;
- wait does not skip scheduled completions;
- wait does not mask repeated idle loops;
- wait exposes why the actor is waiting in debug.

---

## 10. Scheduler and no-human runner requirements

### 10.1 Deterministic actor turn/decision ordering

No-human autonomy must be deterministic. When multiple actors need decisions at the same tick/window, order by stable keys:

1. tick;
2. schedule phase;
3. source kind;
4. actor ID;
5. routine/intention sequence;
6. proposal sequence.

No randomness may be introduced unless it is seeded, logged, replay-safe, and debug-visible. Phase 3A does not need randomness.

### 10.2 Day model

Keep `SimTick` abstract. Do not claim real-world minutes.

The canonical Phase 3A golden day must use deterministic day windows. Recommended default for `no_human_day_001`:

| Window | Tick range | Purpose |
|---|---:|---|
| `pre_dawn` | 0–9 | sleeping/resting completion and wake readiness |
| `morning` | 10–29 | wake, eat, prepare, leave home |
| `work_window` | 30–69 | travel/work block/access blockers |
| `evening` | 70–94 | return home, meal/rest/ordinary fallback |
| `night` | 95–119 | sleep-night routine and end-of-day stability |

A “full day” means all relevant windows are exercised and actors enter an end-of-day/sleep/night stable state. It does not mean 24*60 ticks.

Other fixtures may use other windows only if the fixture schema validates them and tests explain them.

### 10.3 No-human day runner

Add a real no-human ordinary-day runner on top of the existing no-human process skeleton.

Required behavior:

- starts with no bound controller;
- emits no-human day start marker;
- advances windows/ticks deterministically;
- evaluates actor needs/routine windows;
- generates candidate goals;
- adopts/continues intentions;
- submits ordinary proposals through the shared pipeline;
- schedules/handles duration completions;
- detects salient interruptions;
- emits no-human day summary marker;
- records metrics;
- records stuck diagnostics;
- supports deterministic replay/rebuild/projection.

The runner must not produce a day by pre-scheduling a hard-coded list of actor locations. It may schedule actor decision opportunities and duration completions; concrete physical changes must come from action events.

### 10.4 Canonical fixture: `no_human_day_001`

The canonical fixture must include at least:

- `actor_tomas`;
- `actor_mara`;
- `actor_elena`;
- `actor_anna`.

`actor_elias` is optional only if it is cheap and does not pull in Phase 4 report/institution behavior.

The fixture must include:

- homes or home places;
- sleep places;
- a food source that can be consumed;
- at least one missing/inaccessible/insufficient food source for the interruption proof;
- at least one workplace/workstation;
- route/place graph sufficient for ordinary movement;
- routine assignments;
- initial needs;
- day windows;
- expected no-human actor roster;
- expected metrics envelope.

Minimum behavior during a successful no-human day:

- actors wake or transition from night/pre-dawn into morning behavior;
- actors experience need changes;
- actors eat when appropriate and possible;
- actors travel/move by the action pipeline, not teleportation;
- at least one actor completes or fails a work block for modeled reasons;
- actors return/rest/sleep;
- at least one interruption or replanning event occurs;
- at least one physical blocker diagnostic occurs;
- no event references player identity;
- no actor freezes because unpossessed;
- no protagonist gravity affects outcomes;
- replay reconstructs deterministically;
- debug can explain the day.

### 10.5 Recommended actor roles

Keep the roster small. This is a substrate proof, not population simulation.

- Tomas: ordinary worker or shop/stall worker; expected to complete one mundane work block if access and needs permit.
- Mara: hungry household actor; canonical food-unavailable interruption/replan/failure actor.
- Elena: household or workplace actor who exercises travel/return/sleep without being a story puppet.
- Anna: office/work-availability placeholder; no reports/records/institution behavior yet. Her work routine can be “office hours/work block” only.
- Elias, optional: a second workplace/official placeholder only if existing fixture names/docs make him cheap.

Do not add 10–30 agents in this slice. A tiny roster is acceptable if it proves the substrate honestly.

---

## 11. Needs, routines, intentions, and planning requirements

### 11.1 Candidate goal generation

For each autonomous decision opportunity, the engine must evaluate:

- current need bands;
- recent threshold crossings;
- current intention status;
- routine window assignments;
- actor-known relevant places/resources;
- immediate embodied blockers;
- fallback availability;
- planner budget.

The result must be a deterministic set of candidate goals with traceable selection/rejection.

Required candidate goals in Phase 3A:

- eat meal;
- find food;
- sleep/rest;
- go to work;
- perform work block;
- return home;
- continue current intention;
- wait/idle with reason;
- leave unsafe/blocked/unknown situation where safety pressure is active.

### 11.2 Intention commitment and switching

An actor must continue a current active intention unless one of the following occurs:

- intention completes;
- intention fails;
- intention reaches an authored interruption point;
- severe safety pressure demands interruption;
- severe hunger/fatigue crosses an interruption threshold;
- routine window expires and continuation is no longer valid;
- action precondition failure triggers fallback/replan;
- stuck diagnostic transitions the intention to failed/abandoned/suspended.

Switching must produce a trace. Silent switching is forbidden.

### 11.3 Routine method selection

Routine method selection must be HTN-like: use authored methods for ordinary procedures, not pure utility scoring and not hard-coded schedule rails.

A method is selected from known/applicable methods based on:

- goal kind;
- routine assignment;
- actor-known place/resource facts;
- current window;
- current location/body state;
- need thresholds;
- previous failures/fallback attempts;
- deterministic tie-breaking.

If no method applies, the actor must produce a typed failure or fallback. The planner must not synthesize arbitrary story behavior.

### 11.4 Bounded local planning

Bounded local planning is allowed only for concrete action sequencing inside a selected goal/method. Examples:

- route from home to workplace using known/visible adjacent places;
- open a known door before moving;
- check a known pantry before eating;
- move to known sleep place before sleeping;
- select a public food source after home food fails.

Planning constraints:

- bounded budget;
- deterministic expansion order;
- no hidden truth reads;
- no global omniscient route/resource scan unless the actor has authored public knowledge;
- every failure reports `planner-budget-exhausted`, `knowledge`, `physical`, `access`, `resource`, or another typed blocker;
- debug trace records inputs, candidates, selected plan, and rejected steps.

A bounded planner may propose `move`, `open`, `check_container`, `eat`, `sleep`, `work_block`, `wait`, or `continue_routine` actions. It must not directly mutate state.

### 11.5 Routine families

#### MorningWake

Purpose: transition actor from night/pre-dawn rest into ordinary morning decision.

Required behavior:

- if actor is sleeping, wait for scheduled completion or interruption;
- on wake, apply/reconstruct relevant need changes;
- evaluate hunger/fatigue/safety;
- choose continue/eat/work/idle according to candidate goals;
- trace the wake decision.

Morning wake must not teleport actor out of bed/home.

#### EatMeal

Purpose: satisfy hunger using accessible modeled food/service.

Required behavior:

- choose known/believed/visible food source;
- route/open/check if needed through ordinary actions;
- validate access/permission with current Phase 3A rules;
- consume one or more servings only through `eat`;
- reduce hunger eventfully;
- fail/replan if food missing/inaccessible/insufficient;
- record trace.

#### GoToWork

Purpose: travel from current place/home to workplace.

Required behavior:

- choose workplace from routine assignment;
- route by known/visible places;
- submit movement/open proposals;
- fail with physical/access/knowledge diagnostic if route blocked;
- no teleportation.

#### WorkBlock

Purpose: perform a duration-based ordinary work period.

Required behavior:

- start only at valid workplace/workstation;
- schedule duration;
- increase fatigue/hunger appropriately;
- complete/fail eventfully;
- produce non-economic work-completion marker;
- tolerate failure due to need/access/resource with diagnostic.

#### ReturnHome

Purpose: get actor back to home/sleep/evening place.

Required behavior:

- route through ordinary movement;
- fail if blocked with diagnostic;
- may be selected by fatigue/night routine or routine window.

#### SleepNight

Purpose: duration-based sleep.

Required behavior:

- route to sleep place;
- start scheduled sleep;
- reserve body/bed if modeled;
- recover fatigue over time;
- complete/interruption event;
- no instant refill.

#### FindFood

Purpose: fallback when primary meal source fails.

Required behavior:

- use actor-known food memories, visible containers, or authored public food service knowledge;
- may check containers;
- may route to public food service if fixture provides one;
- must not inspect hidden true food location;
- if no valid fallback exists, fail with resource/knowledge/unsupported-action diagnostic.

#### ContinueCurrentIntention

Purpose: preserve durable commitments and let both human and autonomous control continue an actor’s routine.

Required behavior:

- submit next routine proposal or explain why not;
- trace continuation vs switch;
- preserve state across possession;
- fail loudly when no intention or blocked next step.

#### Wait / IdleWithReason

Purpose: represent intentional waiting, not actor freezing.

Required behavior:

- reason is mandatory for autonomous waiting;
- wait duration is bounded;
- wait triggers reevaluation;
- repeated waits without progress trigger stuck diagnostics.

### 11.6 Optional routine families

The following may be added only if cheap and doctrine-aligned:

- `BasicSocialVisitOrTavernStop`;
- `OfficeHoursRoutine`;
- `RespondToImportantContradiction`.

They must remain ordinary placeholder routines. Do not implement speech/report/institution logic in Phase 3A.

---

## 12. Canonical interruption proof

The canonical Phase 3A interruption is:

> food unavailable -> actor replans or fails with typed diagnostic.

Recommended fixture chain:

1. Mara starts the morning with urgent or severe hunger.
2. Mara has an actor-known belief or routine expectation that food is available at a household food source.
3. Mara selects `EatMeal`.
4. Mara routes/checks/opens as necessary through ordinary actions.
5. The household food source is missing, inaccessible, empty, or insufficient.
6. The action/routine does not silently continue, teleport, or refill hunger.
7. The system records:
   - hunger pressure and threshold;
   - selected candidate goal;
   - belief/perception used;
   - selected routine/method;
   - concrete action attempted;
   - action rejection/failure reason;
   - observation/absence/contradiction if applicable;
   - fallback candidate considered;
   - fallback outcome: search, public food, wait, abandon, or typed failure.
8. Debug can explain the whole chain.
9. Replay reconstructs the whole chain.

Recommended success variant for `no_human_day_001`: Mara's home food source is empty, she discovers that, replans to a simple public food service or known neighbor/public pantry, eats, and continues the day. This proves interruption plus recovery.

Recommended negative variant for `food_unavailable_replan_001`: home food is empty and no accessible fallback exists; Mara fails or waits with a resource/knowledge/unsupported-action diagnostic. This proves the engine fails honestly.

Also include a physical blocker diagnostic. Recommended variants:

- workplace locked, causing `GoToWork` or `WorkBlock` failure;
- route blocked by closed/locked door;
- sleep place/bed unavailable.

This blocker may occur in `no_human_day_001` or in `routine_blocked_diagnostic_001`, but Phase 3A must have both the food proof and one physical blocker proof.

---

## 13. Missing-property connection

Phase 3A must preserve the first-proof direction without faking a theft story.

Hard gate:

- no-human ordinary day with needs/routines/interruption/stuck diagnostics.

Preferred/stretch gate:

- `missing_property_setup_no_human_001`, or
- `routine_opportunity_item_move_001`.

Stretch behavior, if implemented:

- an actor moves/takes/places an item for an ordinary modeled reason;
- the action uses the existing take/place pipeline;
- no hidden culprit script;
- no predetermined theft beat;
- no direct item-location mutation;
- no objective quest target.

If autonomous missing-property setup would require broad speech, institutions, report intake, wrong suspicion, or dramatic scripting, defer it to Phase 3B. It is not Phase 4; it is the preferred next ordinary-life proof after Phase 3A.

---

## 14. TUI, view-model, and debug requirements

### 14.1 Embodied view surface

The possessed actor's embodied view must expose Phase 3A mechanics without hidden truth leakage.

Required embodied surface:

- current need/status summary for possessed actor;
- current intention/routine summary in actor-known terms;
- available sleep/eat/work/wait/continue actions;
- why-not explanations for unavailable ordinary-life actions;
- wait/continue time control;
- stopping on possessed-actor-perceived salient interruption;
- no hidden true food location, true routine failure cause, or other actor debug state.

Need values in embodied mode must be banded/labeled, not necessarily exact numbers. Examples:

- “hungry” / “very hungry”;
- “tired” / “exhausted”;
- “uneasy here”;
- “trying to find food”;
- “continuing breakfast routine”;
- “cannot eat: no accessible known food.”

Exact numeric values are debug-only unless the existing TUI style already exposes raw mechanics. The default must favor actor-known status summaries.

### 14.2 Debug surface

Debug must expose the full Phase 3A state.

Required debug surface:

- full need state for all relevant actors;
- current routine/intention for all relevant actors;
- routine step state;
- candidate goals considered;
- selected goal/method;
- rejected goals/methods/actions with reasons;
- planner/decision trace;
- stuck actor diagnostics;
- no-human day summary;
- replay/projection comparison;
- truth/belief mismatch where relevant;
- hidden-truth audit result.

Acceptable debug commands, if commands are needed:

- `debug needs`;
- `debug routines`;
- `debug planner actor_mara`;
- `debug stuck`;
- `debug no-human-day`;
- `debug actor actor_mara`.

Prefer semantic action menu integration for embodied sleep/eat/work/continue. Do not implement ordinary-life actions as raw debug commands.

### 14.3 View-model tests

View-model tests must prove:

- possessed actor sees own need bands;
- possessed actor sees current intention/routine summary;
- possessed actor sees ordinary-life action affordances when valid;
- unavailable actions carry why-not;
- hidden truth is not leaked in embodied view;
- debug view exposes full trace;
- debug view does not mutate actor knowledge;
- no-human run summary is visible in debug/projection;
- replayed projections match original projections.

---

## 15. Replay and projection requirements

Replay must reconstruct Phase 3A state and explain failures. New events/projections must be schema-versioned and deterministic.

### 15.1 Required replay reconstruction

Replay must reconstruct:

- need values or need-changing event effects;
- need threshold crossings that affected decisions;
- candidate goal evaluations where they are eventful/trace records;
- intention starts, suspensions, resumptions, completions, failures, interruptions, abandonments;
- routine step starts/completions/failures;
- sleep/eat/work/wait events;
- autonomous action proposals;
- salient planner traces;
- no-human day start/completion/summary markers;
- stuck diagnostics;
- TUI/debug projections sufficient for snapshot tests.

### 15.2 Required event/projection vocabulary

The implementation may choose exact enum/type names, but equivalent event/projection semantics are required for:

- need delta applied;
- need threshold crossed;
- candidate goals evaluated;
- intention started;
- intention continued;
- intention suspended/resumed;
- intention completed;
- intention failed/abandoned/interrupted;
- routine step started;
- routine step completed;
- routine step failed;
- planner/decision trace recorded or reconstructible;
- sleep started;
- sleep completed;
- sleep interrupted/failed;
- food consumed or food service used;
- eat failed;
- work block started;
- work block completed;
- work block failed;
- continue routine proposed/accepted/rejected;
- stuck actor diagnostic;
- no-human day started;
- no-human day completed/summary.

If the event log would become too noisy, passive need deltas may be batched per window or per meaningful decision interval, but replay must still rebuild the same values and traces.

### 15.3 Replay failure reporting

Replay failure must report:

- event position;
- event kind;
- schema/version issue;
- unsupported event kind;
- missing projection support;
- missing state support;
- checksum mismatch;
- actor/routine/trace ID involved where available.

A replay that silently drops a Phase 3A diagnostic, need event, or routine event is a test failure.

### 15.4 Checksum/projection extension

The existing checksum focuses on physical state. Phase 3A must extend checksum/replay reporting or add complementary checks so need/routine/intention state cannot diverge silently. It is acceptable to keep physical checksum separate if there is an explicit agent-state checksum or replay projection comparison for Phase 3A state.

---

## 16. Content, fixture, and schema validation requirements

Phase 3A fixture/data validation is mandatory. Do not hide ordinary-life setup in Rust-only test helpers.

### 16.1 Schema additions

Extend content schema or fixture schema enough to validate:

- initial need state;
- homes/home places;
- sleep places/beds;
- food stores/items/stacks/services;
- workplaces/work stations;
- routine templates or routine assignments;
- routine preconditions;
- routine steps;
- routine durations/tick windows/day windows;
- routine failure modes;
- interruption triggers;
- fallback declarations;
- expected actor roster for no-human day;
- no-human day fixture manifest;
- Phase 3A action affordances;
- no player-only verbs;
- no hidden outcome chains;
- no schedule teleport fields.

The fixture loader must keep deterministic canonical ordering and reject duplicate or dangling references.

### 16.2 Food schema

Recommended first-slice food model:

- food supply has stable ID;
- food supply has location;
- food supply has integer servings;
- each serving has deterministic hunger reduction;
- access/visibility rules use existing place/container/action machinery where possible;
- consumption decrements servings or marks supply consumed by event;
- zero servings are distinguishable from hidden/unknown food.

A food service may be modeled as an authored simple service only if:

- it has an accessible place;
- it has finite or eventful service-use ancestry;
- it does not introduce prices/markets unless payment is fully custody-aware;
- it is validated as non-quest, non-player-only, non-hidden-outcome content.

### 16.3 Work schema

Recommended first-slice work model:

- workplace ID;
- workplace place/station;
- assigned actors or routine templates;
- work window;
- work duration;
- access preconditions;
- need thresholds affecting availability;
- output tag.

The output tag must be non-economic by default. Examples: `work_block_completed`, `service_completed_placeholder`, `office_available_placeholder`. Do not add wages/debt/payment unless fully eventful and explicitly accepted.

### 16.4 Routine schema

Routine templates or assignments must validate:

- actor/routine references exist;
- every step has a valid kind;
- every concrete action maps to a known registry action or a declared Phase 3A action;
- every routine has at least one failure mode;
- every duration is positive and bounded;
- interruption points are declared;
- fallback/replan behavior is declared or explicitly “fail with diagnostic”;
- no step directly teleports actor/item;
- no step directly sets need values without action/time ancestry;
- no step uses hidden true item location;
- no step references player objective/story beat/culprit/quest fields.

### 16.5 Invalid content examples

The validator must reject fixtures equivalent to:

- actor appears at work at tick 20 with no movement/action ancestry;
- hunger refills without food/service/action;
- sleep restores fatigue instantly without scheduled duration;
- work always succeeds regardless of need/access/place;
- routine has no failure modes;
- routine directly moves item because story needs it;
- planner uses hidden true item location;
- fixture contains objective quest target or player-only field;
- fixture contains `culprit`, `true_culprit`, `stolen_flag`, `npc_knows_truth`, `quest_state`, `player_memory`, or similar hidden outcome shortcut;
- fixture contains schedule teleport fields such as `appear_at`, `force_location_at_tick`, `scripted_absence`, `story_beat`, or equivalent.

The exact forbidden field list must extend the current forbidden-content tests rather than replace them.

---

## 17. Golden fixtures

Phase 3A must add the following golden fixtures or equivalent exact names. Use these names unless there is a hard existing naming collision.

### 17.1 Required fixtures

| Fixture | Purpose | Required proof |
|---|---|---|
| `no_human_day_001` | Canonical no-human ordinary day | wake/eat/move/work/return/sleep, interruption, physical blocker, replay, metrics |
| `ordinary_workday_001` | Focused workday | one actor travels to work and completes or fails a work block for modeled reasons |
| `sleep_eat_work_001` | Action family integration | sleep, eat, work, wait, needs, durations, replay |
| `food_unavailable_replan_001` | Canonical interruption | hunger -> EatMeal -> missing food -> replan/fail with typed trace |
| `routine_blocked_diagnostic_001` | Physical/access blocker | route/workplace/bed blocked -> diagnostic, no silent loop |
| `planner_trace_001` | Trace inspection | candidate goals, selected method, rejected reasons, no hidden truth audit |
| `routine_no_teleport_001` | Anti-shortcut | routine requires movement/action ancestry; validator/runtime catches teleport |
| `possession_does_not_reset_intention_001` | Possession parity | bind/unbind/continue preserves active intention |
| `no_hidden_truth_planning_001` | Epistemic guard | hidden food/item truth cannot influence plan; action may fail and explain |

### 17.2 Stretch fixtures

| Fixture | Purpose | Constraint |
|---|---|---|
| `missing_property_setup_no_human_001` | Ordinary no-human item movement opportunity | only if same take/place pipeline; no culprit script |
| `routine_opportunity_item_move_001` | Actor moves item for modeled ordinary reason | no direct mutation, no story beat |

Stretch fixtures must not block Phase 3A exit unless explicitly promoted by maintainers after the core no-human day passes.

---

## 18. Testing matrix

### 18.1 Unit tests

Required unit coverage:

| Area | Required tests |
|---|---|
| Needs | bounded values, deterministic deltas, threshold bands, threshold crossings, no underflow/overflow |
| Candidate goals | deterministic generation, tie-break stability, active intention durability, severe interruption |
| Intentions | start/continue/suspend/resume/complete/fail/abandon/interruption state transitions |
| Routines | applicability, step transition, duration windows, fallback selection, failure modes |
| Planner | budget exhaustion, no hidden truth input, deterministic expansion order, rejected step reasons |
| Actions | sleep/eat/work/continue validation, started vs rejected vs failed distinctions |
| Reservations | no overlapping body-exclusive actions, bed/workstation conflict if modeled |
| Diagnostics | all blocker categories can be produced and serialized/projection-visible |
| Metrics | counters increment from events, not ad hoc runtime state |

### 18.2 Integration tests

Required integration coverage:

- autonomous proposals use the shared action pipeline;
- human and autonomous proposals share validation results for equivalent actor/action/state;
- no-human runner produces action proposals rather than direct state mutations;
- needs change through wait/sleep/eat/work and survive replay;
- routine continuation submits ordinary next actions;
- route blocked -> routine failure or fallback, not teleport;
- food unavailable -> observation/failure/replan/fail trace;
- work block starts and completes/fails at scheduled tick;
- sleep starts and completes/interruption at scheduled tick;
- possession during an active routine does not reset state.

### 18.3 Golden scenario tests

Required golden scenario assertions:

- `no_human_day_001` has no controller/player-conditioned event facts.
- `no_human_day_001` has at least one meal completed or explicitly missed.
- `no_human_day_001` has at least one sleep completed or intentionally active at end-of-day.
- `no_human_day_001` has at least one work block completed or failed for modeled reason.
- `no_human_day_001` includes the food-unavailable interruption/replan/failure chain.
- `no_human_day_001` includes at least one physical/access blocker diagnostic.
- `no_human_day_001` replay matches final state/projections/metrics.
- `ordinary_workday_001` has movement ancestry for workplace presence.
- `routine_no_teleport_001` fails validation or runtime acceptance if teleport ancestry is missing.
- `no_hidden_truth_planning_001` proves the planner did not use hidden food/item location.

### 18.4 TUI/view-model tests

Required TUI/view-model coverage:

- possessed actor sees needs summary;
- possessed actor sees current routine/intention summary;
- sleep/eat/work/wait/continue appear when valid;
- unavailable sleep/eat/work/continue include why-not;
- embodied view does not reveal hidden food truth;
- debug view shows all actors' needs and routines;
- debug planner view explains Mara’s food failure chain;
- debug stuck view shows blocker category and outcome;
- no-human day summary renders deterministically;
- transcript snapshots cover a possess/wait/continue flow.

### 18.5 Replay tests

Required replay coverage:

- Phase 3A event stream rebuilds needs;
- routine starts/completions/failures replay;
- duration completions replay at same ticks;
- planner traces/projections replay;
- stuck diagnostics replay;
- replay reports unsupported Phase 3A event kinds with event position;
- checksum/projection comparison catches agent-state divergence.

### 18.6 Validation tests

Required content validation coverage:

- valid Phase 3A fixtures load deterministically;
- missing initial need state rejected or defaulted only by explicit validated default;
- invalid home/sleep/food/work references rejected;
- routine with no failure modes rejected;
- routine step with unknown action rejected;
- schedule teleport field rejected;
- hunger refill without food/action rejected;
- instant sleep refill rejected;
- hidden true location/quest/player-only fields rejected;
- no-human fixture missing expected roster rejected;
- no-player-only verbs remains enforced.

### 18.7 Regression/property tests

Required regression/property coverage:

- need values remain bounded;
- no two body-exclusive actions overlap;
- sleep cannot complete without scheduled duration;
- work cannot complete without scheduled duration;
- routine cannot teleport actor or item;
- every meaningful routine transition has event ancestry;
- autonomous proposals use shared action pipeline;
- possession does not copy/reset intention;
- no actor planner reads ground truth;
- no-human events contain no player/controller world fact;
- time advancement does not skip committed events;
- repeated idle loops produce stuck diagnostics;
- debug traces do not mutate actor knowledge.

---

## 19. Metrics and observability

Phase 3A must produce a no-human day metrics/debug artifact. It can be a debug projection, report struct, event-derived summary, or test helper, but it must be deterministic and replayable.

Required metrics:

- events/day;
- routine events/day;
- meals completed;
- meals missed;
- sleep completed;
- sleep interrupted;
- work blocks completed;
- work blocks failed;
- need threshold crossings;
- routine interruptions;
- planner failures;
- stuck actors;
- no-human run duration;
- replay failures;
- TUI action coverage;
- player-conditioned event rate in comparison runs.

Metrics must be derived from event/projection state, not from unlogged side effects. A metric value of zero is acceptable only where the fixture intentionally proves absence. For the canonical no-human day, zero routine events, zero need crossings, zero meals, or zero actor progress is failure.

---

## 20. Explicit non-goals

Spec 0005 explicitly excludes:

- full Phase 3 completion;
- Phase 4 report intake;
- formal incident ledger;
- authority/institution wrong suspicion;
- mature households as institutions beyond minimal domestic access/sleep/food;
- full economy, markets, prices, debt law, wages ledger;
- broad speech/dialogue/report systems;
- gossip/rumor network;
- notices as product feature;
- road travel/regional processes;
- companion recruitment;
- route threat/bounty/proof/payment;
- combat/injury systems;
- LLM agent brains;
- LLM dialogue;
- graphical client work;
- large population simulation;
- procedural town generation;
- quest/objective ontology;
- dramatic director logic.

Do not sneak any of these in under names such as “simple story event,” “temporary script,” “AI director,” “quest target,” “report placeholder,” “motive engine,” or “narrative repair.” If a placeholder is unavoidable, it must be inert, validated, debug-visible, and non-authoritative.

---

## 21. Forbidden shortcuts

The following shortcuts are forbidden and must be caught by review, tests, validation, or debug assertions:

- hard-coded daily schedule teleports actors between places;
- routine step directly mutates actor location;
- routine step directly mutates item location;
- routine step directly lowers hunger without food/service consumption;
- sleep instantly refills fatigue;
- work completes without duration/access/place checks;
- no-human day is just a sequence of waits and marker events;
- planner reads hidden ground truth to pick food/location/action;
- action validator result is used as a planner oracle before proposal;
- debug trace mutates actor knowledge;
- possession resets needs, intentions, or routine state;
- a no-human event references player objective, controller identity, player memory, or quest state;
- failures are swallowed into idle/wait without typed reason;
- repeated idle is accepted as ordinary behavior;
- fixture uses culprit/story/stolen/player-only fields;
- LLM output selects actions or mutates state;
- work output creates money without custody-aware event ancestry;
- a routine succeeds because “the story needs it.”

---

## 22. Failure cases and required outcomes

Phase 3A must define and test failure behavior. Failure is not a bug if it is typed, eventful/replayable, and stable.

| Failure case | Required outcome |
|---|---|
| Actor hungry, no known food | candidate `FindFood` or `EatMeal` rejected/failed with `knowledge` or `resource` diagnostic |
| Actor believes food exists, food absent | check/eat fails, observation/absence ancestry recorded, replan/fail trace |
| Food inaccessible behind locked container/door | `access` blocker, no hunger reduction |
| Actor too fatigued to work | work rejected or interrupted with fatigue/need reason |
| Workplace locked/route blocked | physical/access diagnostic, no teleport |
| Bed unavailable/reserved | scheduling/reservation or resource diagnostic |
| Routine has no next action | unsupported-action or fixture-authoring-error diagnostic |
| Planner budget exhausted | planner-budget-exhausted diagnostic with candidates tried |
| Actor repeats wait without progress | stuck diagnostic and no-human test failure unless fixture expects typed stable wait |
| Possession occurs mid-routine | intention preserved; `continue_routine` continues or explains why not |
| Replay lacks event support | replay failure with event position and unsupported kind |
| Fixture tries schedule teleport | validation rejection |
| Planner hidden-truth test | planner cannot choose hidden correct resource; failure/replan proves actor-known boundary |

---

## 23. Phase exit criteria

Phase 3A is complete only when all of the following are true:

1. The required new needs exist, are bounded, deterministic, and replay-safe.
2. Durable intentions and routine executions exist per actor.
3. Required routine families are implemented with causal action proposals.
4. `sleep`, `eat`, `work_block`/`begin_work`, `continue_routine`, and extended `wait` go through the shared action pipeline.
5. Sleep and work are duration-based.
6. Eating consumes modeled food or service use.
7. Candidate goals and planner traces explain salient autonomous choices.
8. Stuck actor diagnostics are aggressive and typed.
9. `no_human_day_001` runs without a bound controller and exercises wake/eat/move/work/return/sleep/interruption.
10. `no_human_day_001` includes no player-conditioned event facts.
11. `food_unavailable_replan_001` proves the canonical interruption chain.
12. A physical/access blocker diagnostic is proven.
13. Possession does not reset intention/routine/needs.
14. No hidden truth planning test passes.
15. Content validation rejects Phase 3A shortcut fields and malformed routines.
16. Replay rebuilds Phase 3A state/traces/metrics deterministically.
17. TUI embodied and debug surfaces expose Phase 3A mechanics.
18. All required tests pass under the workspace test suite.
19. Metrics/report artifacts are generated from events/projections.
20. No explicit non-goal system has been started as real authority.

### 23.1 Relationship to the full Phase 3 acceptance gate

Phase 3A is a deliberate subset of the Phase 3 acceptance gate (`docs/2-execution/02_PHASE_LADDER_AND_ACCEPTANCE_GATES.md`, "Phase 3 acceptance gates"). The criteria above satisfy every Phase 3 gate line — needs affecting behavior; sleep/food/work/wait/continue through TUI/view models; routines as defeasible intentions with preconditions, duration, failure modes, interruptions, and planner/debug traces; at least one full no-human day; no unpossessed actor freezing; replay of needs/scheduled actions/routine starts/completions/failures/salient traces; and stuck/planner diagnostic events — **except** the gate's "agents … speak minimally …" clause. Speech is deliberately deferred to Phase 3B (§24.1) and excluded here (§20, §11.6). Passing Phase 3A therefore does not by itself close the full Phase 3 acceptance gate; the `speak minimally` line remains open for a later Phase 3 slice.

The exit evidence must be a concise implementation note or test report generated after coding, not additional design philosophy.

---

## 24. Future work after Phase 3A

### 24.1 Phase 3B preferred next slice

Preferred Phase 3B direction:

- autonomous missing-property setup using ordinary needs/routines/opportunity;
- richer routine interruptions;
- more routine variety;
- deeper no-human multi-day run;
- basic ownership/custody pressure and debt/work pressure placeholders;
- stronger food/resource scarcity without market simulation;
- item movement opportunities through the existing take/place pipeline.

Phase 3B must still avoid Phase 4 institutions unless the Phase 3 ladder is explicitly changed.

### 24.2 Phase 2B/3B epistemic depth, if split separately

Remaining epistemic work may include:

- perception depth;
- memory decay;
- testimony;
- speech acts as structured records;
- richer reports/claims;
- gossip/rumor substrate.

These must not be smuggled into Phase 3A except where a tiny actor-known belief/observation is necessary for the food interruption proof.

### 24.3 Phase 4

Phase 4 remains:

- institutions;
- records;
- report intake;
- formal incident ledger;
- wrong suspicion workflows;
- authority procedures;
- household/institution maturity;
- public records/notices where appropriate.

Do not begin Phase 4 in Phase 3A.

### 24.4 Deferred second proof

The second proof involving notices, travel, regional expansion, route threat, bounty/proof/payment, companions, and broader notices remains deferred until after the ordinary-life and missing-property first proof is solid.

---

## 25. Research notes and source notes

Research is design inspiration only. Tracewake doctrine is authoritative.

### 25.1 BDI-style agents

BDI research is useful because it separates beliefs, desires/needs/goals, and intentions. For Tracewake, the important part is not academic purity; it is preventing hidden-truth action and meter jitter. Actors must have subjective beliefs, pressures, and durable commitments. Intentions provide persistence, so an actor does not flip every tick merely because a numeric need changed by one point.

The BDI-planning survey lead also reinforces a practical lesson: predefined plan/recipe libraries are often used for computational efficiency, while planning can add flexibility when recipes do not directly apply. Phase 3A must therefore use authored routine methods first and bounded local planning only for concrete action sequencing/fallbacks.

### 25.2 HTN-style routines

HTN planning is useful where rich domain knowledge and hierarchical procedures matter. Tracewake routines are exactly that kind of domain knowledge: wake, eat, travel, work, return, sleep. Phase 3A must use HTN-like routine templates/methods because they are inspectable, authorable, and bounded. They must remain causal machinery, not scripts.

### 25.3 GOAP/STRIPS-like local planning

The FEAR/GOAP lesson worth borrowing is the decoupling of goals and actions: an actor can have a goal such as “eat” without hard-coding every action sequence in one giant state machine. Phase 3A must adopt the goal/action decoupling and bounded action sequencing, not combat orientation, omniscience, or cinematic behavior. Goals propose actions; actions declare preconditions/effects; the planner searches a tiny actor-known space and can fail.

### 25.4 Game AI failure modes

Phase 3A must explicitly defend against common game AI failures:

- jitter from pure utility every tick;
- schedule rails that teleport or force success;
- omniscient NPCs;
- uninspectable “AI decided” behavior;
- idle loops disguised as autonomy;
- director logic that repairs drama;
- debug panels that reveal but cannot explain.

Durable intentions, routine interruption points, typed planner traces, and aggressive stuck diagnostics are the antidote.

### 25.5 Explainable agents

Explainable-agent research supports the practical requirement that unexpected agent behavior needs inspectable reasons. Tracewake must not settle for “NPC chose action X.” Debug must answer why: need threshold, belief/perception, routine window, candidate goals, selected method, rejected actions, failed precondition, fallback, and hidden-truth audit.

### 25.6 Tracewake-native synthesis

The Phase 3A architecture must be:

- BDI-inspired for separation of beliefs, pressures, and intentions;
- HTN-inspired for authored routine methods;
- GOAP/STRIPS-inspired only for bounded local sequencing;
- Tracewake-native for authority: symbolic, deterministic, eventful, replayable, TUI-visible, debug-explainable, no hidden truth, no LLM brains, no protagonist gravity.

The implementation target is not lifelike psychology. It is boring ordinary life that cannot lie about its causes.

---

## 26. Final implementation directive

Implement Phase 3A as a narrow ordinary-life substrate. Do not write story systems. Do not start institutions. Do not build an economy. Do not add a director. Do not let actors cheat. Do not let failures disappear.

The acceptance bar is simple and brutal:

> Run the no-human day. Replay it. Inspect Mara. Explain the missing food. Explain the blocked workplace or bed. Prove nobody teleported. Prove nobody used hidden truth. Prove possession did not reset intention. Prove every stuck actor is named and diagnosed.
