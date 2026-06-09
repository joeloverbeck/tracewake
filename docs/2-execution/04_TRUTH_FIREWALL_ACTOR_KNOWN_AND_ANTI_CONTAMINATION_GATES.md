# Truth Firewall, Actor-Known, and Anti-Contamination Gates

## Status

Live execution doctrine. This document makes the anti-contamination gate mandatory for every future spec, audit, fixture, and implementation review.

## Authority boundary

This document owns execution-level truth-firewall checks. It does not define new epistemic doctrine; it operationalizes foundation and architecture.

## Depends on

- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`

## Core rule

Truth may validate. Truth may not plan.

Authoritative truth may answer whether a proposed action is physically, socially, procedurally, or resource-valid. It may mutate state only through accepted events. It may produce debug diagnostics. It may not choose an actor's goal, routine method, hidden target, suspect, clue, route, fallback, speech interpretation, institution decision, or embodied affordance.

## Holder-known context

Any cognition or procedure surface must consume a sealed holder-known context. A holder may be an actor, institution, household, LOD aggregate, speech listener, or embodied view model.

A sealed context must contain:

- holder identity;
- tick/window;
- facts, beliefs, observations, records, routines, needs, obligations, and known affordances available to that holder;
- provenance for each input;
- uncertainty, staleness, contradiction, and absence markers;
- explicit unknowns where hidden truth would be tempting;
- an identifier that can appear in proposals, traces, diagnostics, and replay reports.

A sealed context must not contain debug truth, validator truth details, fixture-only truth, global item locations, culprit fields, player memory, true hidden routes, or omniscient record corrections.

## Provenance minimum

Every cognition input must say how the holder can know it:

| Input class | Acceptable provenance examples | Forbidden substitute |
|---|---|---|
| Current place | direct perception, event projection from embodied actor | global body location read with no context boundary |
| Known route | observed exit, remembered route, map record, testimony | true path graph search over unseen edges |
| Food source | seen food, remembered food, record, testimony, search surface | nearest actual food query |
| Workplace | authored assignment with in-world source, record, routine memory | fixture field copied into plan with no source |
| Missing item expectation | direct prior observation, record, testimony, authored prehistory event | true initial item location |
| Suspect possibility | observed trace, rumor, relationship inference, institutional record | culprit flag or theft script |
| Institutional fact | record, role appointment, procedure event, testimony | debug table or schema label |

Routine-template presence is not an information channel; it may organize intent and method selection only after the holder-known context already contains an independently sourced observation, notice, record, memory, or explicit unknown.

## Anti-contamination gate `TFW`

Every future spec and audit must include a `TFW` section with these checks:

1. Name each holder that performs cognition, procedure, affordance selection, speech interpretation, or planning.
2. Name the sealed context builder for each holder.
3. Prove the context excludes validation truth, debug truth, fixture truth, and player-only state.
4. Prove every selected goal/method/affordance cites context inputs.
5. Prove validator truth cannot return suggested hidden targets or actor-visible omniscient reasons.
6. Prove debug reports cannot be parsed by actor-facing code, view models, or tests that claim embodied knowledge.
7. Prove all hidden-truth comparison rows are debug-only audit rows.
8. Prove replay can rebuild the context or the context's source ancestry.

A spec without this section is not admissible for implementation.

## Forbidden contamination patterns

These patterns are invalid even if tests pass:

- Need threshold chooses true nearest food.
- Routine label chooses true workplace without actor-known assignment provenance.
- Scheduler emits a primitive action from raw state.
- Planner searches hidden map edges or unseen containers.
- View model shows an action because truth says it is possible when the actor has no holder-known basis.
- Rejection text reveals the hidden reason in actor-visible fields.
- Debug report is formatted as a string and parsed by gameplay, tests, or embodied UI.
- Content fixture includes `culprit`, `quest_state`, `player_memory`, `npc_knows_truth`, `stolen_flag`, or equivalent fields.
- Institution opens or resolves a case because a fact is true rather than because it received a record, report, observation, or procedure result.
- LLM output creates a fact or plan without deterministic validation and event commitment.

## Positive acceptance evidence

A passing implementation must produce:

- actor-known context trace rows showing selected inputs and rejected candidates;
- hidden-truth audit rows proving excluded truth was available only to debug comparison, not to planning;
- actor-visible why-not outputs free of hidden facts;
- debug outputs labeled non-diegetic;
- replay reports that preserve provenance ancestry;
- negative fixtures where hidden truth exists but cannot be used.

## Human control gate

The world does not know who, if anyone, is controlled by a human. Any branch on human possession must be input-routing, view binding, or debug tooling only. It must not alter knowledge, access, need state, action validity, memory, suspicion, institutional treatment, or simulation priority.

## Review question

For every new behavior, ask:

> Could an autonomous actor with the same holder-known context have reached the same proposal under the same world rules?

If the answer is no, the behavior is contaminated unless the difference is purely a controller input binding or non-diegetic debug operation.
