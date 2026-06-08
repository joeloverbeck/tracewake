# Actor-Known Autonomy Transaction

Proposed architecture clarification doc for Tracewake. Produced by **Spec 0008 — Phase 3A Anti-Contamination Hardening**.

Suggested repository path: `docs/1-architecture/14_ACTOR_KNOWN_AUTONOMY_TRANSACTION.md`

## 1. Purpose

Tracewake autonomy must not be assembled from convenient scheduler shortcuts. An actor’s ordinary action must arise from what the actor can perceive, remember, believe, intend, need, and attempt through ordinary world rules.

This document defines the canonical shape required for Phase 3A and later work:

```text
trigger
 -> actor-known planning context
 -> current needs and active intention
 -> candidate generation
 -> intention continuation/adoption/interruption
 -> HTN/routine method selection
 -> bounded local planning
 -> shared proposal/action pipeline
 -> typed decision/failure/diagnostic records
 -> replay/debug/TUI projection
```

The point is not to add AI sophistication. The point is to prevent hidden-truth contamination and post-hoc explanation.

## 2. Authority

This document clarifies existing foundation and architecture doctrine:

- Actors do not plan from hidden truth.
- Needs are pressures, not scripts.
- Intentions are durable commitments, not labels.
- Routines are defeasible causal machinery, not authored outcome chains.
- The shared pipeline validates world-affecting actions.
- Debug and TUI surfaces render state; they do not author truth.
- Replay must reconstruct meaningful state changes.

If implementation conflicts with these rules, the implementation is wrong.

## 3. Canonical actor-decision transaction

Every autonomous ordinary action must pass through one canonical decision transaction or a directly equivalent architecture.

### 3.1 Inputs

A transaction receives:

- `actor_id`;
- trigger kind, such as no-human window, possessed command continuation, scheduled wakeup, or replan after failure;
- event time and decision window;
- read access to physical state only through perception and validation interfaces;
- live agent state;
- actor epistemic projection or actor knowledge interface;
- content manifest/routine templates;
- shared action registry and pipeline;
- deterministic ordering key.

### 3.2 Stages

A conforming transaction performs these stages in order:

1. Apply due duration completions that occur before this decision.
2. Build actor-known planning context from perception, belief, memory, and authored starting commitments.
3. Read live needs, active intention, routine execution, and reservations.
4. Generate candidate goals from needs, routines, projects, and current intention.
5. Continue, adopt, interrupt, complete, or drop intentions through explicit lifecycle decisions.
6. Select a typed HTN/routine method.
7. Run bounded local planning from actor-known context.
8. Create an ordinary proposal with ancestry IDs.
9. Validate and apply the proposal through the shared pipeline.
10. Emit typed decision trace, routine/intention events, failure/replan/stuck diagnostics, and metrics.
11. Preserve replay/projection agreement.

### 3.3 Forbidden transaction bypasses

No autonomy path may:

- choose food, workplace, route, sleep place, door, container, or movement destination directly from raw physical truth;
- dispatch primitive actions from routine family labels;
- dispatch primitive actions directly from hunger/fatigue thresholds;
- construct an empty epistemic projection and claim actor-known safety;
- append explanation after choosing an unrelated proposal;
- count `continue_routine` as behavioral progress by itself;
- validate live needs from proposal strings;
- store authoritative traces or diagnostics only as display strings.

## 4. Actor-known planning context

### 4.1 Required concept

The planner must consume an actor-known planning context, not raw world truth.

The context is a sealed, provenance-bearing view of what can legitimately affect the actor’s decision. It may include:

- currently visible local facts;
- remembered or believed facts;
- explicit routine/job/home/sleep/work assignment facts;
- fixture-authored possibility-space facts with narrow allowed scope;
- uncertainty and negative observations;
- references to observations, beliefs, events, records, or authored starting commitments.

It may not include validator-only physical truth or debug-only omniscience.

### 4.2 Provenance kinds

The implementation may choose names, but the following distinctions are mandatory:

```text
ObservedNow
  fact came from current local perception/visibility rules

RememberedBelief
  fact came from actor belief/memory/proposition state

RoutineAssignment
  fact came from known duty/job/home/sleep/work assignment state

FixturePossibility
  fact came from authored starting possibility space and is allowed only in limited fixture/planning scopes

PipelineValidationTruth
  fact is authoritative physical truth for validators only; not admitted to planning context

DebugOmniscience
  fact is omniscient debug comparison material; not admitted to planning context
```

### 4.3 Construction rules

- Context fields are private or read-only.
- Fact constructors are restricted.
- Raw physical state can enter only through perception functions that emit provenance.
- Hidden-truth audit walks the provenance graph.
- Booleans such as `actor_known_only=true` are never accepted as proof.
- Display strings are never accepted as proof.

## 5. Needs, intentions, routines, HTN, and local planning

### 5.1 Needs

Needs create pressure. They do not directly script actions.

Severe hunger may cause eating or food-searching only after:

```text
need pressure -> candidate goal -> intention adoption/interruption -> method/local plan -> proposal -> pipeline
```

### 5.2 Intentions

Intentions are durable live commitments. They must persist until explicitly completed, interrupted, failed, replaced, or dropped.

Possession and unpossession must not reset intentions unless an explicit event says so.

### 5.3 Routines

Routines are defeasible method libraries and execution state. They are not guaranteed outcome chains.

A routine may:

- propose candidate duties;
- provide HTN method templates;
- maintain execution progress;
- fail or block;
- be interrupted by higher-priority needs;
- resume after interruption;
- complete through ordinary action ancestry.

A routine may not directly create a primitive action because its family name matches a known case.

### 5.4 HTN and local planning

HTN methods decompose selected goals. Local planning chooses immediate ordinary actions under actor-known context.

The local planner proposes. The shared pipeline validates.

## 6. Shared pipeline validation

Action validators decide whether a proposed action is legal against authoritative state. The planner is not allowed to inspect hidden truth, but validators are allowed to reject a proposal because hidden physical truth makes it impossible.

Validators that need live agent values must read them from validation context. Proposal parameters are not authoritative state.

Examples:

- `work_block` may read actor fatigue/hunger from read-only `AgentState` in validation context.
- `work_block` must not trust `current_fatigue` or `current_hunger` copied into proposal parameters.
- `eat` may validate actual food availability through world state.
- `move` may validate actual adjacency/access.

A validation failure must return structured information to the actor-decision transaction so it can replan, interrupt, or emit a typed stuck diagnostic.

## 7. Typed diagnostics

Tracewake needs typed decision and failure records.

A canonical display string is acceptable only as derived output. It is not authoritative diagnostic state.

A decision trace should identify:

- actor;
- trigger;
- decision window;
- actor-known context/proof graph;
- candidates considered;
- selected candidate;
- active intention before/after;
- selected HTN/routine method;
- local plan;
- proposal;
- pipeline events;
- hidden-truth audit result;
- outcome.

A stuck diagnostic should identify:

- actor;
- transaction stage;
- failure kind;
- actor-known uncertainty;
- missing known affordance;
- pipeline validation failure;
- reservation/body conflict;
- routine/intention references;
- next diagnostic action if any.

## 8. Replay and projections

Replay must rebuild typed autonomy state:

- needs;
- intentions;
- routine executions;
- decision traces;
- stuck diagnostics;
- no-human metrics;
- epistemic/belief/projection state where events support it;
- physical state effects.

Debug reports and TUI panels read typed projections. They do not parse display strings as authoritative facts.

## 9. Acceptance gates for this architecture

A codebase conforms to this document only if tests prove:

1. No-human ordinary actions arise from the canonical transaction.
2. Hidden physical food/routes/workplaces/containers do not affect planning without actor-known provenance.
3. Routine duties and severe needs arbitrate through explicit intention/routine events.
4. `continue_routine` alone does not count as progress.
5. Work validation cannot be forged with proposal need parameters.
6. Typed decision/stuck records survive live/replay comparison.
7. Debug/TUI surfaces derive from typed records.
8. Static/anti-regression tests fail on known forbidden shortcuts.

## 10. Scope boundary

This document does not require Phase 4 institutions, Phase 3B testimony, full economy, regional travel, LLM brains, or a generic AI-framework rewrite.

It requires only the minimum architecture needed to make Phase 3A ordinary no-human life safe to build on.
