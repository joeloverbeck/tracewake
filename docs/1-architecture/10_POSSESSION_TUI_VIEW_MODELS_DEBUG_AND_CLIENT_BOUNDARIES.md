# Possession, TUI, View Models, Debug, and Client Boundaries

## Status

Authoritative architecture contract.

## Purpose / core rule

The TUI is a first-class playable client and acceptance surface. Possession binds human input to an ordinary actor. It does not create a sacred player entity, add player-only verbs, reveal hidden truth, or fork simulation rules.

Embodied view models are actor-legible. Debug view models are non-diegetic. The two must be structurally separate.

## Authority owned

This document owns possession semantics, TUI-first client requirements, embodied view models, actor-legible why-not output, debug-only truth, command loop boundaries, transcripts, test harness obligations, and future client parity.

## Authority denied

The TUI may not apply events, mutate world state, read hidden truth for embodied views, expose debug truth in actor-facing output, or maintain special player state that affects world simulation.

## Contract

### Possession

Possession is controller binding:

```text
controller_id -> bound_actor_id -> mode
```

It is controller/client state, not physical world state. Possession can change which actor receives human input. It cannot reset intentions, needs, memory, routines, location, relationships, institutional status, or knowledge.

A possessed actor acts through the same proposal/validation pipeline as an unpossessed actor.

### Embodied view model

Embodied view models are generated from a holder-known context for the bound actor plus permitted projections.

Minimum embodied view:

```text
embodied_view_model:
  viewer_actor_id
  tick
  place_summary
  visible_exits
  visible_doors
  visible_containers
  visible_items
  visible_actors
  carried_items
  needs_summary
  active_intention_summary
  routine_summary
  notebook_summary
  semantic_action_entries
  last_actor_legible_feedback
  knowledge_context_reference
```

Every semantic action entry must have a stable semantic action ID. Menu positions are presentation-only.

Embodied view models and semantic action entries consume holder-known context
plus permitted projection records whose visible-place, carried-item,
container, current-place, and relevant attribute facts were captured at a
modeled observation, bind/preflight, or perception boundary. The TUI renders
observed labels, attributes, and actor-known affordances from those captured
records. It may not hold a live physical-state handle or re-read truth to
freshen labels, carried-item attributes, routes, workplace availability, food
sources, or hidden blockers.

Authoritative validators may still reject a selected semantic action using
truth, but rejection feedback must split actor-visible modeled feedback from
debug-only detail. The same snapshot/capture rule applies to no-human parity
surfaces and embodied possession: possession is not a knowledge upgrade. This
rule consumes the sealed holder-known context defined by
`03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` and does not decide
whether a future possession boundary emits a modeled perception.

### Semantic action selection

The TUI may parse commands such as selecting a semantic action, waiting, binding a different actor, viewing notebook, running no-human time controls, or opening debug panels. World-affecting commands produce ordinary proposals through the shared pipeline.

No TUI command may mutate authoritative state directly.

### Actor-legible why-not

Actor-facing why-not output must be safe for the actor to know. It may summarize actor-known uncertainty, visible access blockers, lack of perceived target, known resource conflict, unsupported action, or modeled feedback.

It must not include:

- hidden true target;
- culprit identity;
- validator-only state mismatch detail;
- debug source code/internal stage unless actor-legible;
- unexplained knowledge from projections;
- LLM hallucinated facts.

A rejection can have both actor-visible and debug-only reasons. Tests must assert the split.

### Debug view model

Debug view models may show event logs, projection rebuilds, replay reports, physical truth, action rejection internals, truth/belief mismatch, decision traces, stuck diagnostics, hidden-truth audit results, content validation details, and no-human metrics.

Every debug view must be marked non-diegetic and separated from embodied view-model construction. Debug views are not actor knowledge.

### TUI command loop and transcripts

TUI transcripts are acceptance artifacts. They must preserve:

- command input;
- bound actor/mode;
- semantic action IDs;
- proposal IDs and validation reports;
- rendered embodied output when relevant;
- debug markers when debug output is shown;
- event/replay references.

Transcript text is not proof by itself. Tests should assert typed IDs and view-model structures where possible.

### Future clients

Graphical, web, or network clients may replace presentation. They inherit the same view-model/proposal/debug boundaries. No future client may become authoritative simulation, use hidden truth for embodied affordances, or create player-only verbs.

## Required diagnostics / replay / TUI hooks

- Embodied view-model tests must prove actor filtering and stable semantic IDs.
- TUI flow tests must bind, render, submit, rerender, show why-not, run no-human, and inspect debug.
- Debug view tests must prove non-diegetic markers and no reuse as actor knowledge.
- Possession tests must prove controller binding does not mutate world state or reset actor state.
- Command parser tests must prove menu indexes resolve to semantic IDs, not action authority.

## Acceptance implications

A kernel-only simulation is insufficient. Tracewake must remain human-testable through the TUI. A no-human run without TUI/view-model/debug inspection can pass a low-level unit test but not architecture acceptance.

## Anti-patterns

- “TUI is just debug; acceptance can ignore it.”
- “Debug and embodied can share the same data with a `debug` flag.”
- “Player can inspect any location because that is useful.”
- “Human can perform a verb no NPC/process can propose.”
- “Why-not reveals true hidden state to help the player.”
- “Menu index is the action identity.”
- “Binding to an actor resets their intention so the player is not confused.”

## Cross-document obligations

This document consumes holder-known contexts from document 03, action pipeline from document 04, actor transaction summaries from document 05, claims/notebooks from document 06, speech surfaces from document 07, incidents/leads from document 11, and acceptance artifacts from document 13.
