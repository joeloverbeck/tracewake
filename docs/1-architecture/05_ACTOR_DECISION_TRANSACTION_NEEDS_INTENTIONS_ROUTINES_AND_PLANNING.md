# Actor Decision Transaction, Needs, Intentions, Routines, and Planning

## Status

Authoritative architecture contract and canonical actor autonomy transaction.

## Purpose / core rule

Ordinary actors do not act from raw world truth. They act through a sealed actor-known transaction that turns needs, commitments, intentions, routines, obligations, and actor-known possibilities into an ordinary action proposal or a typed stuck/wait/failure diagnostic.

The transaction is not a Phase 3A add-on. It is the architecture spine for actor autonomy and for human possession parity.

## Authority owned

This document owns actor cognition flow: needs, motive pressures, durable intentions, routine/procedure methods, bounded local planning, proposal ancestry, decision traces, stuck diagnostics, and actor-known planning limits.

## Authority denied

The actor transaction may not mutate world state directly. It may not read validation truth to plan. It may not repair itself using debug data. It may not treat routine tables, workplace assignments, home beds, visible containers, or food locations as actor-known without provenance.

## Contract

### Canonical transaction

```text
trigger
 -> sealed actor-known context with provenance
 -> live needs / commitments / intentions / routines / obligations
 -> candidate generation from actor-known pressures only
 -> intention continuation/adoption/interruption/drop
 -> HTN/routine/procedure method selection
 -> bounded local planning from actor-known context
 -> ordinary action proposal with ancestry
 -> shared action pipeline validation against authoritative truth
 -> event commit or rejection/failure semantics
 -> actor-legible feedback and modeled observations
 -> belief/projection updates
 -> typed decision/stuck diagnostics
 -> replay/debug/TUI projections
```

Each arrow is a review boundary. If a stage skips an earlier stage, the implementation is contaminated.

### Trigger

Triggers include tick/window evaluation, need threshold crossing, routine window, active intention reevaluation, interruption, observed contradiction, speech event, institutional notice, danger, scheduled completion, or human possession command requiring actor-world action.

A trigger grants an opportunity to decide. It does not grant hidden knowledge.

### Actor-known context

The transaction consumes a sealed actor-known context from document 03. Candidate generation and planning may use only that context plus live internal actor state that is itself event-sourced or derived from event-sourced agent state.

Required context classes for ordinary life include:

- current embodied place as observed/known;
- known exits/routes/doors and their perceived state;
- known containers/search surfaces;
- known food sources or known ways to search;
- known sleep places/home claims;
- known workplace assignments and whether the actor has a modeled source for them;
- active intention and routine state;
- recent rejection/failure memory;
- known social obligations or notices;
- uncertainty/staleness markers.

### Needs and pressures

Needs are pressures, not scripts. Hunger, fatigue, safety, social obligation, duty, curiosity, fear, or role pressure may produce candidate goals. A need cannot name the true target. “Hungry” may produce “eat known food,” “search known surfaces,” “ask someone,” or “wait/replan.” It may not produce “walk to hidden pantry because food exists there.”

Actor decision transactions consume live need pressures from event-sourced
agent state. Candidate generation may explain why a pressure matters, but it
must not mint need-deltas, supply proposal-side current-need values as
authority, let routine labels charge time, or reconcile duration terminals.
Need accounting and body-exclusive duration lifecycle effects belong to the
single accounting seam owned at the action pipeline / ordinary-life boundary.

### Affect and learned expectations

Affect is a bounded decision influence: a source-bearing salience or pressure
modifier over candidate generation, method selection, interruption,
concealment, confession, accusation, repair tendencies, or routine disruption.
It may explain why a holder prioritizes or avoids an option; it may not reveal
truth, select hidden targets, bypass planning, overwrite beliefs without
events, or force dramatic actions.

A learned expectation is provenance-bearing derived state from remembered
experiences, modeled instruction, records, testimony, repeated failures or
successes, or institution-procedure outcomes. It is not a raw memory, truth
cache, or global probability table unless its source and scope are modeled. It
can influence candidate ordering, method applicability, trust or reliability
judgment, risk aversion, skill confidence, route preference, and routine
adaptation within the same holder-known boundary.

Budget-limited planning may fail or degrade only through typed decision
diagnostics. A bounded planner must not silently choose omniscient shortcuts,
substitute marker actions as progress, or treat budget exhaustion as evidence
that a hidden target, route, method, or social conclusion is available.

### Candidate generation

Candidates must carry:

```text
candidate_goal:
  candidate_goal_id
  actor_id
  source
  priority
  priority_reason
  actor_known_inputs_used
  applicability
  rejection_reason_or_none
  trace_id
```

Rejected candidates are part of the decision trace. Lower-priority choices are evidence, not noise.

### Intentions

Intentions are durable commitments. They reduce jitter and create explainable continuity.

Intention lifecycle states include active, continued, suspended, interrupted, completed, failed, abandoned, and replaced. Transitions require typed reasons and trace ancestry.

An active intention may be continued, interrupted by severe pressure, suspended by a blocker, abandoned after modeled failure, or completed by event evidence. It cannot be silently overwritten by a routine label.

### HTN/routine method selection

Routine methods are reusable task decompositions, not plot scripts. Methods must have applicability conditions and preconditions expressed over actor-known facts, not hidden truth.

Examples:

- `EatMeal` may require actor-known food source or actor-known search surface.
- `GoToWork` may require actor-known workplace and actor-known route surface.
- `SleepNight` may require actor-known sleep place and believed accessibility.
- `ContinueCurrentIntention` may require active intention and next step known.

A selected method emits trace evidence. Rejected methods emit typed rejection reasons.

### Routine and social-rhythm temporal premises

Work, sleep, meals, patrols, appointments, market patterns, household obligations, and institutional appointments are defeasible temporal premises inside actor-known context. Candidate generation and method selection may use those premises only when they arrive through modeled assignment, memory, observation, public cues, records, testimony, institutional context, or modeled inference.

Routine templates may organize method families and expected rhythms, but a template's presence is not itself an information channel. Affect and learning may tune salience or method preference only after the same holder-known premise is available; they cannot create hidden temporal facts.

### Bounded local planning

Local planning converts a selected method and actor-known context into a short proposal sequence or a typed failure. It is bounded by budget, scope, and knowledge. It may use known routes, known doors, known containers, known items, known reservations, and known local affordances. It may not pathfind through undiscovered truth or fetch true targets from global state.

The first proposal in the plan enters the shared action pipeline. Later proposals require reevaluation after events and feedback; a plan is not a script rail.

### Stuck, wait, replan, and failure

Being stuck is a first-class outcome. A transaction that cannot select a valid candidate, method, or local plan must fail closed with a typed stuck diagnostic or a bounded modeled wait. Stuck diagnostics must preserve actor-known explanation and debug-only details separately.

Stuck is not failure of the simulation; it is evidence that the simulation is respecting knowledge boundaries.

### Human possession parity

When a human controls an actor, the actor's embodied view and available actions must be equivalent to what an ordinary actor/process could propose under equivalent known-context conditions. Possession may bind input to an actor; it may not add knowledge, verbs, authority, or outcome protection.

## Data examples

Valid decision ancestry:

```text
decision_trace:
  actor: actor_tomas
  context: ctx_actor_tomas_tick_120
  need_pressure: hunger=severe from event_need_delta_119
  selected_goal: eat_known_food
  selected_method: routine_eat_meal
  local_plan: [check_container(pantry_tomas), eat(food_stew_home_tomas)]
  actor_known_inputs:
    - food_stew_home_tomas observed on table at tick 116
    - route home_tomas->kitchen observed at tick 100
  hidden_truth_audit: actor_known_only
```

Invalid ancestry:

```text
decision_trace:
  actor: actor_tomas
  selected_goal: eat
  local_plan: [move(home_mara), eat(food_hidden_pantry_mara)]
  actor_known_inputs: []
  debug: nearest food found in physical state
```

## Required diagnostics / replay / TUI hooks

- Decision traces must be typed and canonicalizable.
- Stuck diagnostics must include blocker category, actor-known explanation, attempted action, routine/method IDs, fallback/retry status, and debug-only details.
- TUI embodied views must show needs/intention/routine summaries without hidden truth.
- Debug views may show full decision trace and hidden-truth audit explicitly as debug-only.
- No-human reports must include actor decision order, windows, ordinary pipeline event counts, stuck diagnostics, and player/controller contamination metrics.

## Acceptance implications

The missing-property/no-human ordinary-life proof must show unpossessed actors eating, sleeping, working, waiting, failing, or replanning through this transaction. It fails if ordinary day behavior is produced by direct scheduler mutation, routine labels, workplace truth, hidden food truth, or display-string evidence.

## Anti-patterns

- “Hunger above threshold dispatches `eat`.”
- “Routine window dispatches `work`.”
- “Workplace assignment is truth; actor must know it.”
- “Fallback uses true nearest food.”
- “No proposal means emit a harmless continue marker and count progress.”
- “Decision trace says actor_known_only=true but cannot list the facts used.”

## Cross-document obligations

This document depends on document 03 for context sealing, document 04 for proposal/validation, document 06 for epistemic updates, document 10 for possession parity, and document 13 for acceptance artifacts.
