# Authority Boundaries, Rust Workspace, and Dependency Rules

## Status

Authoritative architecture contract.

## Purpose / core rule

Tracewake is a Rust-first authoritative simulation. The kernel owns world truth, events, validation, replay, holder-known context construction, typed diagnostics, and deterministic simulation boundaries. Clients, language surfaces, content packs, debug panels, and tests may request, render, inspect, or validate; they may not become simulation authority.

## Authority owned

The architecture owns logical authority boundaries, not crate names. Current Rust modules are implementation pressure, not doctrine. A conforming implementation may reorganize crates only if it preserves the boundaries below.

| Boundary | Owns | Denied |
|---|---|---|
| Authoritative simulation core | State mutation, event commit, validation, replay, deterministic scheduling, random stream discipline, holder-known context packets | UI-only verbs, hidden director logic, LLM planning, branch-specific behavior |
| Epistemic substrate | Typed propositions, observations, beliefs, memory, records, provenance, holder filtering | Treating records as truth, treating prose as proof, omniscient actor cognition |
| Agent/institution decision layer | Sealed known-context transactions, candidate generation, intentions, procedures, bounded planning, typed diagnostics | Direct raw-state reads for cognition, goal repair from validator truth |
| Action pipeline | Ordinary proposals, validation, rejection/failure semantics, event construction, feedback events | Proposal bypass, content mutation, player-only action path |
| Content/domain packs | Possibility space, seeds, initial records, routines, norms, fixtures, schemas, validation constraints | Direct authoritative mutation at runtime, authored outcomes, guaranteed clues/rewards |
| Projection/view layer | Read models, TUI view models, debug views, metrics, story sifting | New facts, mutation authority, embodied debug leakage |
| Client surfaces | Input binding, rendering, transcripts, command parsing | Event application, hidden truth planning, special player verbs |

## Authority denied

No subsystem may smuggle authority through naming, convenience, or phase scope. The following are forbidden even in tests:

- a scheduler that turns a need, routine label, workplace assignment, or fixture fact directly into a primitive action;
- a TUI command that bypasses ordinary actor proposal/validation;
- a content pack that executes mutation logic;
- a projection that writes truth back into authoritative state;
- an LLM call that creates facts, chooses plans, validates truth, or repairs narrative;
- a debug panel that feeds embodied cognition or actor-facing affordances.

## Contract

### Dependency direction

Authoritative dependencies flow inward toward the kernel. Information surfaces flow outward through projections.

```text
content/domain packs -> validation/loaders -> authoritative state seeds
TUI/human input      -> ordinary proposal -> shared pipeline -> event log
scheduler/processes  -> known-context transaction -> ordinary proposal -> shared pipeline
LLM/language surface -> parse/render draft -> validation -> structured speech/action
kernel event log     -> projections/view models/debug/replay/story sifting
```

Reverse dependencies are forbidden. The kernel must not depend on terminal UI libraries, graphical clients, hosted LLM APIs, analytics dashboards, external time, mutable global randomness, or branch-local test harness assumptions.

### Rust-first boundary

The simulation core must be executable and testable without a graphical engine, server process, hosted model, or human controller. Rust is the authority for authoritative events, state, validation, context packets, diagnostics, replay, and deterministic no-human runs.

A future engine or service may render the world, request proposals, host terminals, or store files. It may not become the source of truth for what happened.

### TUI-first boundary

The TUI is a first-class playable client, not a disposable debug shell. Architecture must keep the TUI close enough to the kernel to exercise embodied views, actor-known affordances, why-not explanations, notebooks, no-human time controls, and debug quarantine.

A future graphical client must conform to the same view-model and proposal boundaries. It may improve presentation; it may not receive privileged truth.

### Content/domain-pack boundary

Content packs define possible objects, places, routines, norms, records, fixtures, and initial conditions. They are inputs to validation and seed construction. They cannot invoke event application, alter state during play, or author guaranteed outcomes.

Allowed:

- a household record saying Tomas expects coins in his strongbox;
- a workplace assignment with a modeled source, role, and visibility;
- a routine template that can fail;
- a notice board artifact with an author, posting event, and stale date;
- a scenario pressure such as an empty pantry.

Forbidden:

- a fixture flag that makes an actor go to the true hidden food source;
- a quest step that spawns a clue if the player falls behind;
- a “culprit” field that actor cognition can read;
- a content-scripted sanction;
- a narrative beat with mutation authority.

### Side effects and storage

Authoritative side effects must be explicit boundary inputs or event log writes. Wall-clock time, file-system order, hash-map iteration accidents, live API responses, network status, and OS randomness cannot influence simulation outcome unless represented as committed boundary events with schema/version/provenance.

### Display strings are surfaces

Display labels, summaries, transcript text, and LLM prose are renderings. They may help humans inspect a result, but they are never authoritative proof. Tests and review artifacts must assert typed IDs, event kinds, propositions, reason codes, provenance references, context hashes, and canonical serialized diagnostics.

## Data/projection examples

A minimal content seed may say:

```text
record_seed:
  record_id: record_tomas_strongbox_expectation
  proposition: item_expected_at(container=strongbox_tomas, item=coin_stack_01)
  holder: actor_tomas
  source: household_memory_seed
  visibility: actor_private
```

It may not say:

```text
quest_truth:
  culprit: actor_mara
  next_clue: spawn_if_player_stuck
```

## Required diagnostics / replay / TUI hooks

- All boundary crossings that can affect simulation must be event-sourced or explicitly included in replay manifests.
- All holder-known context packets must expose source classes, source IDs, and context hashes.
- All decisions must emit typed decision/stuck diagnostics when they select, reject, wait, fail, replan, or fall back.
- TUI transcripts must use stable semantic action IDs, not menu positions as proof.
- Debug panels must carry non-diegetic markers and never feed embodied view models.

## Acceptance implications

A test that demonstrates correct world mutation but cannot show the ordinary proposal, validation report, event(s), holder-known context ancestry, and replay result does not pass. A no-human run that produces plausible schedules but cannot prove actor-known decision ancestry fails.

## Anti-patterns

- “The TUI knows the target, so the actor can act.”
- “The fixture says this person works here, so the scheduler can move them there.”
- “The validator knows the door is locked, so it can recommend the key.”
- “The LLM explanation sounds right, so the belief is established.”
- “Debug view and embodied view are the same struct with a flag.”
- “A content pack can run a tiny script just for first proof.”

## Cross-document obligations

This document feeds document 02 for event/replay boundaries, document 03 for known-context authority, document 04 for pipeline authority, document 10 for client boundaries, and document 13 for acceptance evidence.
