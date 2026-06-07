# Spec 0004 — Phase 2A Epistemic Substrate, Expectation Contradiction, and Possession Parity Implementation Specification

Status: PROPOSED  
Repository: `joeloverbeck/tracewake`  
Target commit analyzed: `c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056`  
Spec filename: `0004_PHASE_2A_EPISTEMIC_SUBSTRATE_EXPECTATION_CONTRADICTION_AND_POSSESSION_PARITY_IMPLEMENTATION_SPEC.md`  
Phase: Phase 2A, opening Phase 2 work without broadening into Phase 3 or Phase 4

This specification uses the user-supplied commit as the target of record. It does not verify that the commit is the current `main`.

## 1. Evidence ledger of exact URLs fetched

The uploaded manifest was used only as a path inventory for the target commit. The following exact file URLs were fetched and read successfully for this specification:

- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/README.md
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/docs/1-architecture/02_ACTION_AFFORDANCE_SCHEDULING_AND_PROPOSAL_PIPELINE.md
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/docs/1-architecture/03_EVENT_LOG_REPLAY_PROJECTIONS_AND_SAVE_PACKAGES.md
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/docs/1-architecture/05_CLAIMS_BELIEFS_TRACES_MEMORY_AND_INFORMATION_FLOW.md
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/docs/2-execution/02_PHASE_LADDER_AND_ACCEPTANCE_GATES.md
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/docs/2-execution/03_FIRST_PROOF_MISSING_EXPECTED_PROPERTY.md
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/docs/2-execution/05_PHASE_1_KERNEL_TUI_EVENT_LOG_AND_REPLAY.md
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/docs/4-specs/SPEC_LEDGER.md
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/archive/specs/0003_PHASE_1A_EXECUTABLE_TUI_COMMAND_LOOP_AND_DOC_ALIGNMENT_SPEC.md
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/crates/tracewake-core/src/state.rs
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/crates/tracewake-core/src/events/envelope.rs
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/crates/tracewake-core/src/events/apply.rs
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/crates/tracewake-core/src/actions/proposal.rs
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/crates/tracewake-core/src/actions/report.rs
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/crates/tracewake-core/src/actions/pipeline.rs
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/crates/tracewake-core/src/actions/registry.rs
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/crates/tracewake-core/src/actions/defs/takeplace.rs
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/crates/tracewake-core/src/view_models.rs
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/crates/tracewake-core/src/projections.rs
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/crates/tracewake-content/src/schema.rs
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/crates/tracewake-content/src/validate.rs
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/crates/tracewake-content/src/fixtures/strongbox_001.rs
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/crates/tracewake-tui/src/app.rs
- https://github.com/joeloverbeck/tracewake/blob/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/crates/tracewake-tui/src/input.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/docs/2-execution/06_PHASE_2_EPISTEMICS_VIEW_MODELS_AND_POSSESSION_PARITY.md

Unrelied failed exact-file attempts, included here for completeness because they were exact owner/repo/SHA/path attempts but produced no trusted content:

- https://raw.githubusercontent.com/joeloverbeck/tracewake/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/c7e562513ab2f1fe75dcfbf1453e4aa9cf49e056/docs/4-specs/SPEC_LEDGER.md

No branch URL, default-branch lookup, repository metadata lookup, GitHub code search, snippet search, or clone was used as evidence for this specification.

## 2. Executive summary

The repository is post-Phase-1 and post-Phase-1A. Phase 1 delivered the physical/action/event/TUI/replay spine. Phase 1A delivered the executable stdin/stdout command loop. The spec ledger explicitly says the next implementation spec may open Phase 2 work, and the Phase 2 execution document defines the gate as belief-before-truth, observation-versus-interpretation, actor-known filtering, and possession parity.

This spec therefore defines **Phase 2A**, not all of Phase 2. It adds the smallest executable epistemic substrate that proves Tracewake's central claim:

```text
belief comes before truth
```

The required proof is narrow and playable:

```text
Mara can move or take Tomas's coins as an ordinary actor.
The human/debug controller can switch to Tomas.
Tomas does not inherit Mara's knowledge or the human's cross-possession knowledge.
Tomas has a source-backed expectation that coin_stack_01 should be in strongbox_tomas.
Tomas intentionally checks/searches strongbox_tomas.
The system records a channel-specific observation of contents/absence.
That observation contradicts Tomas's expectation.
Tomas receives a source-backed missing-property belief.
Tomas's actor-known notebook shows only Tomas-known observations and beliefs.
Embodied view hides hidden truth.
Debug view shows ground truth, belief/truth mismatch, and provenance.
Replay rebuilds physical and epistemic projections deterministically.
```

Phase 2A also includes the minimum sound-observation slice required by Phase 2 doctrine:

```text
Elena may receive a low-confidence sound observation near Tomas's room or strongbox.
That observation must not become knowledge of theft.
It may support only uncertain propositions such as sound heard near place or possible movement nearby.
```

Phase 2A does **not** implement reports, records, institutions, wrong suspicion, gossip, routines, daily life, notices, travel, companions, combat, LLM dialogue, or a graphical client.

## 3. Doctrine alignment

### 3.1 Governing invariants

Phase 2A is required by the constitutional invariants:

- **Belief-before-truth (`INV-002`):** actors act from modeled beliefs, observations, memory, records, testimony, and inference, not from hidden ground truth.
- **No privileged protagonist (`INV-004`):** the authoritative world ignores human existence; controller binding is metadata, not a sacred world entity.
- **Possession transfers no world knowledge (`INV-006`):** the human may remember other bodies, but the current actor may not report, accuse, testify, navigate, or otherwise act from facts not acquired through modeled channels available to that actor.
- **Absence needs a channel (`INV-016`):** absence becomes evidence only through expectation, perception, instruction, or intentional search.
- **Typed propositions are the epistemic currency (`INV-021`, `INV-022`):** raw prose may render claims, but it may not be authoritative state.
- **Important beliefs need provenance (`INV-026`):** important beliefs record holder, proposition, stance, confidence, source, channel, acquisition time, staleness/verification, contradiction links, and scope.
- **Embodied and debug surfaces are separate (`INV-067`, `INV-068`):** embodied mode shows actor-known reality; debug mode may show truth but must be visibly non-diegetic and must never satisfy actor knowledge.

### 3.2 Phase ladder alignment

The Phase 2 execution gate requires:

- observation events for direct perception, search/touch, simple sound, and absence where expectation exists;
- beliefs with holder, proposition, stance, confidence, source, acquisition time, and channel where relevant;
- expectation contradiction that does not trigger for actors with no expectation;
- embodied view models containing only actor-known/perceived/source-backed facts;
- debug view models that reveal truth only as non-diegetic inspection;
- possession switch changing input binding only;
- a player acting as Mara, switching to Tomas, and Tomas being unable to truthfully accuse from human memory;
- no-human perception/belief updates;
- replay rebuilding observations, belief updates, contradictions, and actor-known snapshots.

Phase 2A implements the first executable slice of that gate. It intentionally leaves Phase 2B+ room for richer perception, memory decay, testimony, speech, reports, records, institutions, and wrong suspicion.

The Phase 2 execution gate's "candidate goals: search, ask, report, conceal, suspect" outcome of contradiction (see `docs/2-execution/06` §expectations-and-absence-as-evidence and the first-proof chain in `docs/2-execution/03` §5) is **partially adjudicated** here. Phase 2A surfaces leads only as actor-known **notebook projections** over the holder's beliefs/observations/contradictions (a source-bound projection, consistent with `INV-059` — leads and tasks are projections, not ground-truth objectives), and not as quest-like goals. Planner-driven candidate-goal generation that *changes actor behavior* depends on the needs/intentions/HTN substrate and is deferred to Phase 3 (`docs/2-execution/07`).

### 3.3 First-proof alignment

The first proof is Missing Expected Property. Phase 2A covers only the early epistemic segment of that proof:

```text
property expected somewhere
 -> property moved or taken
 -> absence discovered through intentional check/search
 -> observation contradicts expectation
 -> source-backed belief and notebook entry
 -> possession parity proves no knowledge leak
 -> debug explains truth/belief mismatch
```

It does not proceed to report intake, incident ledger, institutional reasoning, or wrong suspicion. Those belong to Phase 4.

## 4. Current code baseline

Repository analysis at the target commit confirms the supplied interpretation.

### 4.1 Workspace and executable surface

The workspace has the three-crate shape:

```text
tracewake-core     authoritative simulation kernel
tracewake-content  fixtures, content loading, schema validation
tracewake-tui      stdin/stdout TUI boundary
```

The dependency direction remains `tracewake-tui -> tracewake-content -> tracewake-core`; `core` must not depend on higher layers.

The README describes Phase 1 as landed: a runnable physical/action/event/TUI/replay spine. The default TUI loads `strongbox_001`, binds `actor_tomas`, supports `view`, `bind`, numeric menu selection, `do <semantic_action_id>`, `wait`, debug log/bindings/item/rejection/projection/replay, and clean quit.

### 4.2 Core state

`crates/tracewake-core/src/state.rs` currently has first-class physical state for:

- actors;
- places;
- doors;
- containers;
- items;
- institution and record placeholders;
- controller binding metadata;
- an `OwnershipCustody` stub with an `expected_location` placeholder.

It does not yet have first-class propositions, observations, beliefs, contradictions, notebooks, knowledge contexts, or epistemic projections.

### 4.3 Events and replay

`crates/tracewake-core/src/events/envelope.rs` currently defines event streams for world, diagnostic, controller, and replay-debug events. Current event kinds cover controller attach/detach, actor movement, door/container open/close, item movement, wait/time advance, action start/fail/reject, no-human advance, and replay projection rebuild.

`crates/tracewake-core/src/events/apply.rs` applies physical world events to `PhysicalState` and treats non-world events as no-ops for physical state. Unsupported event schema versions are rejected.

There are no epistemic event kinds yet, such as observation recorded, belief seeded, belief updated, or expectation contradicted.

### 4.4 Action pipeline

`crates/tracewake-core/src/actions/pipeline.rs` already contains the architectural slots Phase 2A needs:

- `KnowledgePerceptionPlaceholder`;
- `SocialNormPlaceholder`;
- `CostDurationCheck`;
- `ReservationConflictCheck`;
- `PhaseBoundaryValidation`;
- `MutationPlanConstruction`;
- event construction, append, application, projection/view update, and debug linkage.

The comment in the pipeline confirms these slots are deliberately inert for later epistemic, norm, cost/duration, and reservation systems. Phase 2A must activate the epistemic slot without creating a parallel rule engine.

`crates/tracewake-core/src/actions/registry.rs` currently supports `QueryOnly`, `Move`, `Open`, `Close`, `Take`, `Place`, and `Wait`. There is no `check_container`, `search_container`, or accusation/knowledge-basis probe.

`crates/tracewake-core/src/actions/report.rs` has physical/controller reason codes but no `KnowledgePreconditionNotMet` reason.

### 4.5 View models and TUI

`crates/tracewake-core/src/view_models.rs` currently has an embodied physical view model and debug view models for controller binding, event log, item location, action rejection, projection rebuild, and replay report. It has no actor-known notebook, no belief/observation view, and no debug epistemics view.

`crates/tracewake-core/src/projections.rs` builds embodied views from physical locality. It hides closed-container contents physically, but there is no explicit `KnowledgeContext` and no source-backed actor-known belief filtering.

`crates/tracewake-tui/src/input.rs` supports `help`, `view`, `bind`, `do`, numeric selection, `wait`, and the existing debug commands. It has no `notebook`, `debug beliefs`, `debug observations`, or `debug epistemics` commands.

### 4.6 Content schema and fixtures

`crates/tracewake-content/src/schema.rs` currently serializes actors, places, doors, containers, items, and affordances. There is no fixture schema for initial beliefs, expectations, propositions, or observation seeds.

`crates/tracewake-content/src/validate.rs` validates IDs, references, locations, topology, state, action registry parity, semantic view constraints, no-player, no-script, determinism hazards, fixture contract, and serialization round-trip. Its raw fixture line parser accepts only physical Phase 1 tags. It must be extended for Phase 2A epistemic seed data and must continue rejecting player/quest/story shortcuts.

`crates/tracewake-content/src/fixtures/strongbox_001.rs` is currently a Phase 1 physical fixture with Tomas and Elena, a house, a street lane, a door, a closed strongbox, and `coin_stack_01`. It intentionally asserts no belief contradiction, suspicion, report, institution, quest, reward, player, or outcome script. Phase 2A must add a Phase 2 variant or extension rather than corrupting the Phase 1 baseline.

## 5. Phase 2A purpose

Phase 2A exists to cross the line from a physical toy box into Tracewake's epistemic product identity.

The slice is deliberately small. It must answer the following questions in executable form:

1. Can the engine represent typed claims/propositions without stringly global flags?
2. Can an actor receive a channel-specific observation without automatically receiving interpretation or truth?
3. Can an actor hold a source-backed belief or expectation that is private to that holder?
4. Can a missing expected item become evidence through intentional check/search?
5. Can the actor-known notebook expose only the current actor's source-backed beliefs and observations?
6. Can debug show hidden truth and belief/truth mismatch without leaking it into embodied mode?
7. Can possession switch the controller without transferring knowledge between actors?
8. Can replay rebuild both physical and epistemic projections?
9. Can a non-human/scheduler-origin actor perform the same epistemic check through the same proposal/event/projection machinery?

The answer must be yes before Phase 3 ordinary-life routines or Phase 4 institutions begin.

## 6. Entry requirements

Phase 2A may start only if the following remain true at implementation time:

- The Phase 1/1A workspace still builds as a three-crate Rust workspace.
- The command loop from Spec 0003 remains executable with `cargo run -p tracewake-tui`.
- The existing Phase 1 golden fixtures still load and validate.
- The existing action pipeline remains the only world-affecting mutation path.
- The event log remains append-only.
- Replay of physical projections still passes existing gates.
- Debug panels remain visibly non-diegetic.
- The content validator still rejects quest/player/direct-script shortcuts.
- The TUI still selects actions through stable semantic action IDs, not menu positions as identity.

If any of these fail during implementation, fix the regression inside the Phase 2A work before claiming the phase slice complete. Do not proceed by adding a second epistemic path beside the existing pipeline.

### 6.1 Assumptions and open implementation choices

These are one-line-correctable decisions the implementer must settle; the spec deliberately leaves them open but surfaces them so they are decided rather than discovered:

- **Seed mechanism for initial beliefs** — §9.3 offers two acceptable approaches (pre-tick-0 epistemic projection seeded as content manifest, or deterministic pre-start `InitialBeliefSeeded` events). Either is allowed if it satisfies the replay identity `same fixture + same event log -> same belief projection`.
- **Check timing** — §10.3 allows the `check_container` action to resolve immediately or to cost one tick. Either is allowed if the tick change is deterministic and visible in the event log/replay.
- **Epistemic event placement** — §9.1 allows a new `EventStream::Epistemic` or epistemic events inside the existing world stream, provided physical application never silently mutates beliefs and epistemic projection application is explicit and replay-tested.
- **Sound-slice origin** — §11.4 allows Elena's sound observation to be produced at runtime from Mara's action (preferred) or seeded as authored prehistory; either must carry a channel/source, never a truth assertion.

## 7. Deliverables

Phase 2A must deliver the following systems as one coherent implementation:

1. typed proposition/claim substrate;
2. observation model;
3. source-backed belief store;
4. initial expectation fixture schema;
5. intentional container check/search action;
6. expectation contradiction and absence-as-evidence;
7. `KnowledgeContext` or equivalent actor-known filtering;
8. actor-known notebook projection and TUI/view-model access;
9. debug epistemics view;
10. possession parity proof;
11. knowledge-blocker why-not reporting;
12. short deterministic no-human epistemic run;
13. replay/projection rebuild for epistemic state;
14. Phase 2A golden fixtures and regression tests;
15. documentation updates required by this spec.

These are deliverables, not separate ticket files.

## 8. Data model requirements

### 8.1 Core module placement

Add the epistemic substrate to `tracewake-core`, not to the TUI or content crate.

Recommended module shape:

```text
crates/tracewake-core/src/epistemics/
```

The exact file decomposition is implementation detail, but the public API must make these concepts first-class:

- proposition/claim;
- observation;
- belief;
- contradiction;
- knowledge context;
- actor-known notebook projection;
- debug epistemics projection;
- canonical serialization and stable ordering helpers;
- replay/projection builders.

The TUI must consume view models. It must not implement epistemic rules.

### 8.2 Stable IDs

Add stable ID types or equivalent validation for:

- `PropositionId` if propositions are stored independently;
- `ObservationId`;
- `BeliefId`;
- `ContradictionId`;
- `EpistemicProjectionVersion` or equivalent schema marker.

These IDs must follow the existing stable ID discipline:

- deterministic;
- non-display names;
- no whitespace;
- ordered deterministically;
- safe in canonical serialization;
- never derived from hash-map iteration order.

### 8.3 Typed proposition model

Add a structured proposition representation. Prefer an enum-like domain model over raw strings.

Minimum proposition variants:

```text
ItemLocatedInContainer(item_id, container_id)

ItemLocatedAtPlace(item_id, place_id)

ItemCarriedByActor(item_id, actor_id)

ContainerContentsObserved(container_id, observed_present_item_ids, observed_absent_item_ids)

ItemMissingFromExpectedLocation(item_id, expected_location)

SoundHeardNearPlace(place_id, tick_window, sound_profile)

PossibleMovementNearPlace(place_id, tick_window, support_observation_id)

ActorWasNearPlace(actor_id, place_id, tick_window)
```

`ActorWasNearPlace` is optional for Phase 2A and may be test/debug-only. It must not be used to smuggle culprit truth into Tomas's embodied knowledge. Elena's sound observation must not create `ActorWasNearPlace(actor_mara, ...)` unless an actor-specific modeled channel actually supports it.

`expected_location` must be a structured location, not prose. It may refer to:

- container;
- place;
- actor carriage;
- later, record/institutional locations.

For Phase 2A, container and place are sufficient.

The proposition model must be:

- canonically serializable;
- comparable for equality and contradiction checks;
- renderable into human-readable text without making prose authoritative;
- reference-validated against known actor/place/container/item IDs;
- schema-versioned or projection-versioned enough for replay to reject unsupported forms.

### 8.4 Observation model

Add first-class observations as event-backed epistemic inputs.

Minimum fields:

```text
observation_id
observer_actor_id
channel
observed_tick or tick_window
observer_place_id
subject or target reference
raw_payload
confidence
source_event_id / source_action_id / source_process_id
optional alternatives
content/projection schema version
privacy/scope
```

Minimum channels for Phase 2A:

```text
direct_sight
touch_or_search
simple_sound
absence_marker
reading_placeholder_schema_only
```

The reading placeholder is allowed only as a schema-level future hook. It must not implement records, notices, or report intake in Phase 2A.

**Confidence encoding:** `confidence` here and on beliefs (§8.5) must use a deterministic canonical encoding — a bounded integer scale or fixed-precision decimal string, not a raw floating-point `Display`. Floating-point formatting is a determinism hazard for canonical serialization, checksums, and replay (`INV-017`, `INV-018`; see §13.4 and the existing `validate_determinism` discipline). Source doc `docs/2-execution/06` illustrates confidence as a float (`0.34`, `0.86`); the implementation must pin a deterministic representation of that value.

Observation is not interpretation. The observation layer records what reached an actor through a channel. It does not automatically conclude motive, culprit, theft, ownership, legal proof, or institutional fact.

Required examples:

```text
Tomas checks strongbox_tomas and observes present items: []
Tomas checks strongbox_tomas and, because he expected coin_stack_01 there, records observed absence of coin_stack_01.
Elena hears brief low-confidence metal/wood sound near house_tomas around tick N.
```

Forbidden example:

```text
Elena heard a sound, therefore Elena knows Mara stole the coins.
```

### 8.5 Belief model

Add holder-specific beliefs.

Minimum fields:

```text
belief_id
holder_id
holder_kind
proposition
stance
confidence
source_kind
source_id
channel
acquired_tick
believed_event_tick if known
last_verified_tick if relevant
contradiction_links
privacy/scope
content/projection schema version
```

Minimum holder kinds for Phase 2A:

```text
actor
institution_placeholder
```

Only actor holders are required to be active. Institution placeholder support may exist only to avoid a future schema break; no institutional procedure may be implemented.

Minimum stances:

```text
believes_true
believes_false
expects_true
plausible
doubts
unknown_or_unresolved
```

For Phase 2A, the essential stances are `expects_true`, `believes_true`, and `plausible`.

Important beliefs may not lack holder or source. A belief is not a global proposition flag.

Required Tomas expectation:

```text
holder: actor_tomas
proposition: ItemLocatedInContainer(coin_stack_01, strongbox_tomas)
stance: expects_true
confidence: high enough to matter
source_kind: authored_prehistory or prior_direct_observation
source_id: stable fixture/prehistory/source ID
acquired_tick: before scenario start or tick 0 seed
```

Required Tomas contradiction result:

```text
holder: actor_tomas
proposition: ItemMissingFromExpectedLocation(coin_stack_01, InContainer(strongbox_tomas))
stance: believes_true
confidence: derived from search/check confidence
source_kind: observation
source_id: observation_tomas_checked_strongbox
channel: touch_or_search or direct_sight
contradiction_links: expectation belief and observation/contradiction ID
```

### 8.6 Contradiction model

Add explicit contradiction records or equivalent projection links.

Minimum fields:

```text
contradiction_id
holder_actor_id
prior_belief_id
observation_id
new_belief_id if created
proposition_expected
proposition_observed
contradiction_kind
detected_tick
confidence
source_action_id or source_event_id
```

Minimum contradiction kind:

```text
expected_item_absent_from_container
```

Optional future-proof kinds may be present as inert vocabulary, but Phase 2A must not implement social/institutional contradiction machinery.

### 8.7 KnowledgeContext

Add a `KnowledgeContext` or equivalent object used by embodied view models and any ordinary planner/action precondition that depends on knowledge.

Minimum fields:

```text
viewer_actor_id
mode
current_tick
allowed_sources
forbidden_sources
perception_scope
belief_scope
observation_scope
projection_schema_version
```

Allowed embodied sources:

- current perception;
- actor's own observations;
- actor's own beliefs;
- actor memories if implemented later;
- actor's own actions;
- actor-heard speech if implemented later;
- actor-read records if implemented later.

Forbidden embodied sources:

- unobserved event-log truth;
- hidden item location;
- debug causal graph;
- other actors' private beliefs;
- controller possession history;
- human/debug notes;
- previous possessed actor knowledge.

Debug mode uses a different context and must remain visibly non-diegetic.

### 8.8 Epistemic projection state

Do not store beliefs as global flags on physical entities. Add an epistemic projection alongside physical state.

Minimum projection contents:

```text
observations_by_id
observations_by_actor
beliefs_by_id
beliefs_by_holder
contradictions_by_id
contradictions_by_holder
notebook_entries_by_actor
projection_version
event_range_or_checkpoint
content_manifest_id
```

The projection must be rebuildable from:

- the initial fixture/content manifest;
- ordered event log;
- supported event/projection schema versions;
- deterministic rules.

The physical state remains authoritative for current item location. The epistemic projection represents what actors know or believe.

## 9. Event model requirements

### 9.1 Event stream policy

Add an epistemic event stream or equivalent clearly classified nonphysical authoritative stream.

Recommended addition:

```text
EventStream::Epistemic
```

If the implementation instead places epistemic events in the existing world stream, it must still separate physical application from epistemic projection application. Physical `apply_event` must not silently mutate beliefs. Epistemic projection application must be explicit, deterministic, and replay-tested.

### 9.2 New event kinds

Minimum new event kinds:

```text
InitialBeliefSeeded
ObservationRecorded
BeliefUpdated
ExpectationContradicted
ContainerChecked
```

`ContainerChecked` may be represented as an action outcome event, or its information may be entirely represented by `ObservationRecorded` plus the validation report. However, the event log must answer that Tomas intentionally checked/searched the strongbox. A mere view render or open-container event is not enough.

Optional but useful:

```text
SoundObservationRecorded
```

This may be a specialization or a `ObservationRecorded` event with channel `simple_sound`.

### 9.3 Initial fixture beliefs and event sourcing

Initial beliefs may be loaded as part of the canonical initial content state. They must still have structured provenance:

```text
source_kind: authored_prehistory
source_id: prehistory_tomas_checked_strongbox_before_start
```

Two acceptable implementation approaches:

1. The fixture creates a deterministic initial epistemic projection before tick 0, with seeded beliefs treated as part of the content manifest.
2. The loader emits deterministic pre-start `InitialBeliefSeeded` events before regular tick 0 events.

Either approach must satisfy replay:

```text
same fixture + same event log -> same belief projection
```

If seeded beliefs are not ordinary events, the replay report must include the content manifest and initial epistemic projection version. The engine must reject mismatched content/projection versions rather than silently inventing a repaired belief store.

### 9.4 Observation event payload

Observation event payloads must include:

```text
observation_id
observer_actor_id
channel
observed_tick or tick_window
observer_place_id
target_id or subject reference
proposition(s) directly observed where applicable
raw_payload
confidence
source_action/proposal/event/process
alternatives
schema_version
```

For a container check, the observation must record the container contents observed by the actor through the modeled channel. For absence-as-evidence, it must also record which expected item was absent, but only when the actor had a relevant expectation.

### 9.5 Belief update event payload

Belief update events must include:

```text
belief_id
holder_id
proposition
stance
confidence
source_kind
source_id
channel
acquired_tick
prior_belief_id if updated
contradiction_ids
privacy/scope
schema_version
```

The event must be enough for replay to rebuild the belief projection without consulting hidden current truth except through event payloads and initial content.

### 9.6 Expectation contradiction event payload

Expectation contradiction events must include:

```text
contradiction_id
holder_actor_id
expectation_belief_id
observation_id
expected_proposition
observed_or_absence_proposition
resulting_belief_id if any
detected_tick
source_action_id or proposal_id
schema_version
```

The contradiction must link Tomas's prior expectation to Tomas's own check/search observation. It must not link debug truth directly to Tomas's belief.

### 9.7 Sound observation payload

The Elena sound slice must represent uncertainty. Minimum payload:

```text
observer: actor_elena
channel: simple_sound
observer_place: place where Elena is
perceived_origin_place: house_tomas or room near strongbox
tick_window
sound_profile:
  intensity: low
  duration: brief
  material_hint: metal_or_wood or unknown
confidence: low
alternatives:
  house_settling
  person_moving
  object_shift
  misheard_sound
```

Allowed belief update:

```text
SoundHeardNearPlace(...)
stance: believes_true or plausible, depending on confidence policy
confidence: low
source: observation_elena_sound_...
```

Allowed derived proposition:

```text
PossibleMovementNearPlace(...)
stance: plausible
confidence: low
```

Forbidden belief update:

```text
Mara stole coin_stack_01
Mara was in Tomas's room
coin_stack_01 was stolen
```

unless those claims have their own actor-known sources in a later phase.

## 10. Action and pipeline requirements

### 10.1 New action family

Add an intentional container epistemic action.

Stable action ID:

```text
check_container
```

Stable semantic action ID shape:

```text
check.container.<container_id>
```

TUI label:

```text
Check <container label or ID>
```

`search_container` may be added later or as an alias if implementation wants to distinguish quick check from thorough search. Phase 2A requires at least `check_container`.

Do not make `open` or `inspect_entity` automatically solve epistemics. Opening a container may make contents visible physically, but it must not by itself create source-backed absence evidence or a missing-property belief. The actor must intentionally check/search, or another modeled channel must create an observation.

### 10.2 Registry and effect

Extend the action registry with an epistemic-capable effect such as:

```text
ActionEffect::CheckContainer
```

or an equivalent action definition that can produce nonphysical epistemic events.

The existing `QueryOnly` effect is not enough if it leaves no event, because Phase 2A observations and belief changes are meaningful eventful state. `check_container` must pass through the shared proposal/validation/event pipeline and append events.

### 10.3 Validation requirements

`check_container` must validate:

- actor exists and is enabled;
- actor is at the required place;
- container exists;
- container is reachable from actor's current place;
- container is open or its contents are visible enough for the declared check method;
- locked/closed state is respected;
- the action uses an actor knowledge context where applicable;
- proposal origin may be human, scheduler/test, or agent, using the same pipeline;
- controller binding is required only for human-origin proposals;
- no hidden truth or debug memory is used as actor knowledge;
- duration/tick cost is applied if existing action timing supports it.

For Phase 2A, the check may be immediate or one tick. If it advances time, the event log and replay must show the deterministic tick change. Do not invent full search duration/reservation/interruption machinery beyond the current scheduler's capacity.

### 10.4 Outcome requirements

A successful `check_container` must:

1. append an event or event sequence that proves the check occurred;
2. record a container-contents observation for the actor;
3. compare the observation against that actor's relevant expectations;
4. create contradiction links if the expectation is contradicted;
5. create or update a source-backed missing-property belief for that actor;
6. update actor-known notebook projection;
7. leave hidden truth hidden in embodied mode;
8. expose debug truth/belief mismatch in debug mode.

The Phase 2 execution gate also names contradiction-derived "candidate goals" (search, ask, report, conceal, suspect). In Phase 2A these are exposed only as actor-known notebook leads (a projection per `INV-059`), not as behavior-driving goals; planner-driven candidate-goal generation is deferred to Phase 3 (see §3.2).

### 10.5 Negative cases

Required negative behavior:

- Actor checks an empty container but has no expectation: observation may record contents, but no missing-property contradiction or missing-item belief is created.
- Actor expected the item in a different location: no contradiction for this container check.
- Actor checks a closed/inaccessible container: action is rejected or physically blocked; no hidden contents observation is created.
- Actor checks a locked container without access: blocker is physical/access-related, not hidden truth.
- Actor has only Elena's low-confidence sound observation: actor may know sound/possible movement, not theft or culprit.
- Tomas cannot truthfully accuse Mara from human memory or debug truth.

### 10.6 Knowledge-blocker why-not

Add a new validation reason category:

```text
KnowledgePreconditionNotMet
```

Stable ID:

```text
knowledge_precondition_not_met
```

Minimum use case:

```text
Tomas cannot truthfully accuse Mara because Tomas has no actor-known source linking Mara to the missing coin.
```

Phase 2A must not implement full accusation consequences, speech, reports, suspicion, institutional response, or social memory effects.

Required implementation path:

- Add a validation helper that checks whether an actor has a source-backed belief/observation/proposition supporting a requested claim.
- Add a minimal query/probe action if needed for TUI/regression coverage.

Recommended probe action ID:

```text
truthful_accuse_probe
```

The probe must be `QueryOnly` or non-mutating. It exists to exercise why-not and knowledge filtering. It must not commit speech, accusation, reputation, suspicion, or institutional effects.

If exposed in TUI, it must produce an embodied why-not summary that does not leak hidden truth:

```text
Tomas knows the coins are missing, but has no actor-known source linking Mara to them.
```

Debug may expand:

```text
Ground truth: Mara took coin_stack_01. Tomas lacks modeled information connecting Mara to that event.
```

### 10.7 Controller parity

Human-origin proposals still require controller binding. Scheduler/test/agent proposals must not require controller binding. This already exists in the pipeline and must be preserved.

Phase 2A must add tests proving:

```text
same action definition
same validation path
same event/projection machinery
different proposal origin
no player-only behavior
```

## 11. Fixture and content schema requirements

### 11.1 Schema extension

Extend `FixtureSchema` with a Phase 2A epistemic seed section.

Minimum new schema data:

```text
initial_beliefs or initial_expectations
```

Minimum fields for each seeded belief/expectation:

```text
belief_id
holder_actor_id
proposition
stance
confidence
source_kind
source_id
channel
acquired_tick
last_verified_tick if relevant
privacy/scope
schema_version
```

The schema may call the section `beliefs`, `initial_beliefs`, or `expectations`; the final name must be consistent across schema, serialization, validation, fixtures, and tests.

### 11.2 Required Tomas expectation fixture data

Phase 2A must extend or variant `strongbox_001` with:

```text
belief_id: belief_tomas_expects_coin_stack_01_in_strongbox_tomas
holder: actor_tomas
proposition: ItemLocatedInContainer(coin_stack_01, strongbox_tomas)
stance: expects_true
confidence: high
source_kind: authored_prehistory or prior_direct_observation
source_id: prehistory_tomas_checked_strongbox_before_start
channel: memory or direct_sight
acquired_tick: before scenario start or 0
```

This must be content data, not hard-coded test-only state.

### 11.3 Required Mara content

Current `strongbox_001` does not include Mara. Phase 2A must introduce Mara in the possession parity fixture:

```text
actor_mara
```

Mara must be an ordinary actor at a reachable place. Her ability to move/take coins must use existing physical actions and pipeline validation. The fixture must not include a `culprit` field, `stolen_flag`, `npc_knows_truth`, or any equivalent shortcut.

### 11.4 Required Elena sound fixture data

The sound slice may be produced by a deterministic event during the test sequence rather than seeded in fixture content. If seeded for a fixture, it must be an observation/belief seed with a channel/source, not a truth assertion.

Allowed seed shape:

```text
Elena has no theft knowledge.
Elena may have no initial sound observation.
During the run, when Mara moves/takes coins, a deterministic sound observation event may be produced for Elena if the fixture places her nearby.
```

or:

```text
Elena starts with a low-confidence authored-prehistory sound observation near Tomas's room.
It supports only SoundHeardNearPlace or PossibleMovementNearPlace.
```

Prefer runtime observation from Mara's action if cheap. Seed only if necessary for a bounded implementation.

### 11.5 Content validation rules

The validator must reject:

- important belief missing holder;
- important belief missing source kind or source ID;
- belief proposition referencing unknown item/container/place/actor;
- expectation pointing to unknown item or container;
- duplicate belief IDs;
- duplicate observation IDs if observations can be seeded;
- unstable ordering of beliefs/observations after canonicalization;
- unknown proposition variant;
- unsupported epistemic schema version;
- raw prose belief content with no typed proposition;
- shortcut truth fields.

Extend the raw-line allowlist to include Phase 2A epistemic sections only after validation is implemented. Do not weaken existing no-player/no-script validation.

Explicit forbidden fixture fields or equivalents:

```text
quest
objective
reward
player
player_character
player_memory
npc_knows_truth
culprit
true_culprit
stolen_flag
quest_state
knows_mara_did_it
scripted_discovery
auto_report
auto_suspicion
```

The exact raw serialized field names may differ, but the validator must reject these concepts.

### 11.6 Serialization and canonicalization

Epistemic fixture data must round-trip through the same content serialization discipline as physical fixture data.

Requirements:

- deterministic field order;
- deterministic collection order;
- stable ID sorting;
- no reliance on hash-map iteration order;
- content validation round-trip test;
- changed order in source fixture produces same canonical form;
- unsupported schema version rejects loudly.

## 12. TUI, view-model, and debug requirements

### 12.1 Embodied view model

Extend `EmbodiedViewModel` or add an actor-known companion view consumed by the TUI.

Minimum new embodied data:

```text
knowledge_context_id or viewer actor marker
notebook_summary_available
salient_actor_known_beliefs
recent_actor_known_observations
known_contradictions
source/confidence labels
```

This data must come from the actor's `KnowledgeContext`, not direct ground truth.

Do not dump the full debug belief store into embodied mode. The embodied notebook is actor-filtered.

### 12.2 Actor-known notebook

Add a real actor-known notebook projection.

Required TUI command:

```text
notebook
```

If the implementation uses a view-model harness rather than the stdin/stdout TUI for some assertions, the command still must be either implemented or explicitly equivalent through the same renderer/input layer used by TUI tests. Phase 2A should not be headless-only.

The notebook must show:

- source-bound beliefs;
- recent observations;
- confidence;
- acquisition tick/time;
- last verification tick if available;
- contradiction links;
- expectation contradiction summary;
- actor-known possible leads only if directly derivable and not quest-like.

It must exclude:

- hidden truth;
- debug notes;
- previous-body knowledge;
- omniscient culprit labels;
- other actors' private beliefs;
- actor-unknown event-log facts.

Recommended notebook wording for Tomas after checking:

```text
Notebook — actor_tomas
Observation: You checked strongbox_tomas and did not find coin_stack_01.
Belief: coin_stack_01 is missing from its expected location.
Source: your check of strongbox_tomas.
Confidence: high, bounded by search/check confidence.
Contradicts: your earlier expectation that coin_stack_01 was in strongbox_tomas.
Unknown: who moved it, why it is absent, whether it was stolen, borrowed, misplaced, or misremembered.
```

The exact rendering may differ, but the content boundaries must hold.

### 12.3 Debug epistemics view

Add debug-only inspection for epistemic state.

Required TUI commands:

```text
debug epistemics
debug beliefs <actor_id>
debug observations <actor_id>
```

`debug epistemics` may include all actors; `debug beliefs` and `debug observations` focus by holder/observer.

Debug may show:

- ground truth item location;
- all observations;
- all beliefs by holder;
- contradiction links;
- truth/belief mismatch;
- possession binding history as non-world metadata;
- event/projection rebuild details;
- why embodied view does not show a fact.

Debug must be visibly non-diegetic. The renderer must label it with the existing debug convention or an equivalent visible marker:

```text
DEBUG NON-DIEGETIC: Epistemics
```

Debug inspection must never write to actor notebooks.

### 12.4 TUI command parser

Extend `UiCommand` and `DebugCommand` parsing for:

```text
notebook
debug epistemics
debug beliefs <actor_id>
debug observations <actor_id>
```

The parser must preserve existing commands and tests. It must reject bad actor IDs and bad debug command shapes with typed errors.

### 12.5 Renderers and panels

Add renderers for:

- actor-known notebook;
- debug epistemics summary;
- debug beliefs by actor;
- debug observations by actor.

Rendering must be deterministic and testable by transcript/snapshot tests. Avoid table order dependence on hash-map iteration.

### 12.6 Embodied leak tests

Every embodied view and notebook test must assert absence of hidden truth strings where possible.

Examples of strings that must not appear in Tomas's embodied view/notebook before modeled discovery:

```text
actor_mara took coin_stack_01
Mara stole the coins
culprit
debug
human saw
previous possession
```

After Tomas checks the strongbox, the notebook may say the coins are missing from expected location. It still must not say Mara did it unless a modeled actor-known source supports that claim.

## 13. Replay and projection requirements

### 13.1 Physical and epistemic projection split

Replay must rebuild:

- physical state using existing physical event application;
- observations;
- belief updates;
- expectation contradictions;
- actor-known notebook projection;
- truth/belief mismatch debug projection;
- controller binding history as non-world metadata.

Do not treat the notebook as authoritative. It is a projection over beliefs, observations, contradictions, and source metadata.

### 13.2 Epistemic projection builder

Add a deterministic epistemic projection rebuild path.

Minimum inputs:

```text
initial content fixture / initial epistemic seed data
event log
supported event schema registry
supported epistemic projection schema version
content manifest ID
```

Minimum outputs:

```text
epistemic projection
actor-known notebook projection
debug epistemics projection
replay/projection report
```

### 13.3 Unsupported versions

Replay must reject:

- unknown epistemic event type;
- unsupported epistemic event schema version;
- unsupported proposition schema version;
- unsupported observation/belief projection version;
- missing source references;
- missing holder references;
- content manifest mismatch where the initial belief seed is required.

Do not silently repair missing source IDs, invalid holders, unknown proposition variants, or old schema versions.

### 13.4 Determinism

Repeated replay of the same fixture and event log must produce identical:

- observation ordering;
- belief ordering;
- contradiction ordering;
- notebook ordering;
- debug epistemics ordering;
- projection checksum/report output.

Use ordered maps/sets or explicit sorting. Do not rely on hash-map iteration order.

### 13.5 Possession stream

Controller binding remains non-world metadata. Replay/debug may show it. Embodied views and notebooks must not treat possession history as actor knowledge.

## 14. Golden fixtures and scenarios

Phase 2A must add or extend fixtures/tests with the following names. Each fixture contract must state setup, allowed actions, expected events/projections, acceptance assertions, and forbidden shortcuts.

### 14.1 `strongbox_001` Phase 2 variant or extension

Purpose: extend the Phase 1 physical strongbox baseline with Tomas's source-backed expectation without corrupting the Phase 1 fixture's physical baseline.

Setup:

- `actor_tomas` exists.
- `strongbox_tomas` exists and can be checked.
- `coin_stack_01` is a physical item.
- Tomas has a source-backed expectation that `coin_stack_01` is in `strongbox_tomas`.
- No actor has culprit truth.
- No report/record/institution/suspicion fields are present.

Allowed actions:

- open/close strongbox through existing physical action path;
- check `strongbox_tomas`;
- view notebook;
- debug epistemics.

Expected events/projections:

- initial belief seed or deterministic initial epistemic projection;
- check action produces observation;
- if coin absent, contradiction and missing-property belief;
- notebook projection for Tomas;
- debug epistemics projection.

Forbidden shortcuts:

- no `culprit`;
- no `stolen_flag`;
- no `quest_state`;
- no player memory;
- no global `coin_missing_known` flag.

### 14.2 `expectation_contradiction_001`

Purpose: prove absence-as-evidence.

Setup:

- Tomas expects `coin_stack_01` in `strongbox_tomas`.
- The physical state at check time has `coin_stack_01` absent from `strongbox_tomas`.
- Tomas has no source linking Mara or anyone else.

Allowed actions:

- check/search strongbox;
- notebook;
- debug epistemics.

Expected events/projections:

- `ObservationRecorded` for contents/absence;
- `ExpectationContradicted`;
- `BeliefUpdated` for `ItemMissingFromExpectedLocation`;
- Tomas notebook shows missing property with source;
- debug shows physical location and mismatch.

Acceptance assertions:

- missing-property belief is holder-specific to Tomas;
- belief has source and confidence;
- contradiction links expectation and observation;
- no culprit claim appears in Tomas embodied view/notebook.

Forbidden shortcuts:

- empty container alone does not create global truth;
- debug item location does not become Tomas belief.

### 14.3 `possession_parity_001`

Purpose: prove human cross-possession knowledge does not transfer.

Setup:

- `actor_mara`, `actor_tomas`, and optionally `actor_elena` exist.
- Mara can reach `strongbox_tomas` under ordinary physical rules.
- Tomas has the source-backed expectation.
- The controller may bind Mara and then Tomas.

Allowed actions:

- bind Mara;
- move/open/take/place as available;
- bind Tomas;
- view/notebook;
- check strongbox;
- truthful accusation probe;
- debug epistemics.

Expected events/projections:

- controller binding events in controller stream;
- Mara's physical item movement events;
- Tomas's notebook initially lacks Mara-only knowledge;
- Tomas's check creates observation/contradiction/missing belief;
- accusation probe rejects for knowledge precondition;
- debug reveals truth/belief mismatch and possession history.

Acceptance assertions:

- possession switch creates no world knowledge transfer;
- Tomas cannot truthfully accuse Mara from human memory;
- debug can show Mara's action and Tomas's ignorance;
- Mara remains an ordinary actor.

Forbidden shortcuts:

- no copied beliefs from Mara to Tomas;
- no human/debug note as source;
- no player-only verb.

### 14.4 `view_filtering_001`

Purpose: prove same event log supports different actor-filtered views.

Setup:

- At least Tomas and Mara.
- Mara knows her own action through her own action event or observation.
- Tomas does not know Mara's action before modeled channel.
- Debug knows physical truth by inspection.

Allowed actions:

- build embodied view for Mara;
- build embodied view for Tomas;
- build debug epistemics view.

Expected projections:

- Mara view may contain actor-known own action if modeled.
- Tomas view does not contain Mara-only knowledge.
- Debug view contains truth and mismatch.

Acceptance assertions:

- actor-known filtering differs by actor over same event log;
- debug truth never appears in embodied mode.

### 14.5 `knowledge_blocker_accuse_001`

Purpose: prove knowledge-precondition why-not.

Setup:

- Tomas knows coins are missing after checking.
- Tomas has no actor-known source linking Mara.
- Debug truth may show Mara moved/took coins.

Allowed actions:

- run `truthful_accuse_probe` or equivalent validation helper;
- render why-not;
- render debug expansion.

Expected reports:

- `KnowledgePreconditionNotMet`;
- embodied summary does not leak hidden truth;
- debug expansion may show ground truth and missing modeled source.

Acceptance assertions:

- no speech/social effect committed;
- no suspicion score;
- no report;
- no institution.

### 14.6 `sound_uncertainty_001`

Purpose: prove simple sound observation remains uncertain.

Setup:

- Elena is near enough to receive a sound observation.
- A physical action near strongbox/room can create a low-confidence sound observation, or an authored prehistory sound observation is seeded.
- Elena does not see Mara take the coin.

Allowed actions:

- trigger or load sound observation;
- view Elena's notebook;
- debug observations/beliefs.

Expected events/projections:

- sound observation for Elena;
- optional low-confidence `SoundHeardNearPlace` belief;
- optional low-confidence `PossibleMovementNearPlace` belief;
- no theft knowledge.

Acceptance assertions:

- Elena's notebook does not say Mara stole coins;
- Elena's observation has alternatives and low confidence;
- debug shows source event/action if runtime-generated.

### 14.7 `no_human_epistemic_check_001`

Purpose: prove epistemic mechanics run without a controller.

Setup:

- No controller bound.
- A deterministic scheduler/test/agent proposal causes an actor to check/search a container.
- Actor has a relevant expectation.

Allowed process:

- load fixture;
- submit scheduler-origin check/search through the same action pipeline;
- rebuild projections;
- replay.

Expected events/projections:

- no controller/human/player references in world or epistemic events;
- observation recorded;
- belief/contradiction updates occur;
- notebook projection can be built afterward;
- replay rebuilds same epistemic state.

Acceptance assertions:

- same action/event/projection machinery as human path;
- no direct state mutation;
- no special no-human cheat.

## 15. Test requirements

### 15.1 Unit tests

Add unit tests for:

- proposition canonicalization;
- proposition equality/comparison;
- observation creation;
- observation does not equal belief;
- sound observation alternatives/confidence;
- belief insert/update;
- belief source required;
- important belief holder required;
- expectation contradiction detection;
- absence requires expectation;
- expectation in different location does not contradict current check;
- actor-known filtering;
- debug view sees truth while embodied view does not;
- knowledge-blocker why-not reason code;
- unsupported epistemic schema version rejection;
- duplicate belief/observation IDs rejected.

### 15.2 Property and regression tests

Add regression/property tests for:

- no important belief lacks holder/source;
- debug truth never appears in embodied view or notebook;
- possession switch never copies beliefs;
- same event log supports different actor-filtered views;
- actor cannot truthfully claim/accuse from hidden truth;
- replay rebuilds belief projection;
- seeded expectations are deterministic regardless of fixture source ordering;
- event/projection ordering is deterministic;
- physical item location remains single-source while beliefs may diverge.

### 15.3 Content validation tests

Add tests that reject fixtures with:

- belief missing holder;
- belief missing source;
- proposition referencing unknown item/container/actor/place;
- expectation pointing to unknown item or container;
- duplicate belief IDs;
- unstable ordering;
- shortcut truth fields:
  - `npc_knows_truth`;
  - `culprit`;
  - `true_culprit`;
  - `quest_state`;
  - `stolen_flag`;
  - `player_memory`;
  - `knows_mara_did_it`;
  - equivalents.

Add tests that accept the Phase 2A strongbox expectation fixture and produce canonical deterministic output.

### 15.4 Action/pipeline integration tests

Add integration tests for:

- `check_container` passes through shared pipeline;
- human-origin check requires matching controller binding;
- scheduler-origin check does not require controller binding;
- check inaccessible/closed container rejects physically;
- check empty expected container creates observation, contradiction, belief;
- check empty unexpected container creates observation only;
- check different expected location creates observation only;
- action rejection appends meaningful rejection event where current debug convention requires;
- knowledge blocker emits `KnowledgePreconditionNotMet`.

### 15.5 TUI and transcript tests

Add or update TUI/transcript tests covering:

```text
bind actor_mara
move/open/take or otherwise move coin_stack_01 using stable semantic actions
bind actor_tomas
view
notebook
check strongbox_tomas
notebook
debug epistemics
debug beliefs actor_tomas
debug observations actor_tomas
debug replay
```

Required assertions:

- before Tomas checks, Tomas's view/notebook lacks Mara-only knowledge;
- after Tomas checks, notebook shows missing-property belief with source;
- notebook does not identify Mara as culprit;
- debug epistemics shows truth/belief mismatch;
- replay/debug projection still passes;
- numeric selection still resolves through current view to stable semantic action IDs.

### 15.6 Golden scenario tests

Add golden tests for all Phase 2A fixture names:

```text
expectation_contradiction_001
possession_parity_001
view_filtering_001
strongbox_001 Phase 2 variant/extension
knowledge_blocker_accuse_001
sound_uncertainty_001
no_human_epistemic_check_001
```

Existing Phase 1 golden tests must continue passing.

## 16. Acceptance gates

Phase 2A is accepted only when all of the following pass.

### 16.1 Mechanic gate

- Typed propositions exist and are canonical.
- Observations exist as event-backed epistemic inputs.
- Beliefs exist as holder/source-backed state.
- Expectation contradiction works for Tomas's missing coin.
- Sound observation remains uncertain and non-culprit-bearing.

### 16.2 TUI/view-model gate

- `notebook` or the equivalent TUI-facing actor-known view is reachable.
- `debug epistemics`, `debug beliefs <actor_id>`, and `debug observations <actor_id>` are reachable.
- Tomas's embodied view and notebook hide hidden truth.
- Debug visibly shows truth/belief mismatch.
- TUI does not implement simulation or epistemic rules.

### 16.3 Possession parity gate

- Human controls Mara.
- Mara moves/takes `coin_stack_01` through ordinary actions.
- Human switches to Tomas.
- Tomas does not inherit Mara/player knowledge.
- Tomas discovers absence only through modeled check/search/observation.
- Tomas cannot truthfully accuse Mara without actor-known support.
- Debug can show truth and possession history as non-diegetic metadata.

### 16.4 No-human gate

- A scheduler/test/agent-origin actor can check/search without controller metadata.
- Observation and belief updates occur without human/controller references.
- Replay rebuilds no-human epistemic projections.
- No event references `player` or human identity as world cause.

### 16.5 Replay gate

- Replay rebuilds physical state and epistemic projections.
- Unknown/unsupported event/proposition/observation/belief/projection versions reject loudly.
- Actor-known notebook projection rebuilds deterministically.
- Debug truth/belief mismatch projection rebuilds deterministically.

### 16.6 Validation gate

- Content validates Phase 2A expectation fixtures.
- Content rejects belief/source/proposition/shortcut failures.
- Serialization round-trip is deterministic.
- Existing Phase 1 content remains accepted unless intentionally migrated with tests.

### 16.7 CI gate

The implementation is not done until all existing workspace gates pass:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

## 17. Explicit non-goals

Phase 2A must not include:

- Phase 3 needs/routines/full no-human day;
- hunger/fatigue/work/sleep implementation beyond existing placeholders;
- motive-driven autonomous theft;
- full social speech acts;
- reports;
- records;
- local authority;
- institutions;
- wrong suspicion;
- suspicion scoring;
- gossip mutation;
- rumor networks;
- memory decay/distortion beyond schema placeholders;
- LLM dialogue;
- LLM agent brains;
- notices;
- roads;
- travel;
- companions;
- combat;
- large region;
- graphical client;
- procedural village generation;
- freeform parser;
- quest/objective tracker;
- reward/payment/proof flow.

These are later gates.

## 18. Forbidden shortcuts

The implementation must not:

- store `MaraDidIt` as actor knowledge;
- store `culprit` or `stolen_flag`;
- use debug truth to satisfy embodied action preconditions;
- copy beliefs between actors on possession switch;
- treat human memory as actor memory;
- let `open container` automatically create missing-property belief;
- make a container's emptiness evidence to actors with no expectation;
- create Elena theft knowledge from a sound;
- let the TUI inspect hidden truth in embodied mode;
- write actor notebook entries from debug panels;
- use raw prose as authoritative proposition content;
- hard-code Tomas's expectation only in tests;
- directly mutate belief projection outside event/projection machinery;
- make a no-human run mutate state through a special test-only shortcut;
- silently accept unsupported schema/event/projection versions.

## 19. Modeled failure cases

Required failure cases:

- actor checks a closed container whose contents are not visible;
- actor checks a locked/inaccessible container;
- actor checks a container at another place;
- actor checks an empty container with no expectation;
- actor checks the wrong expected location;
- observation has no belief update because confidence or expectation rules do not support one;
- belief seed references an unknown item;
- belief seed lacks source;
- belief seed lacks holder;
- duplicate belief IDs;
- proposition variant unsupported by replay;
- `truthful_accuse_probe` lacks actor-known support and rejects with knowledge blocker;
- debug truth exists but embodied mode hides it;
- possession switch does not transfer notebook entries;
- replay sees unsupported epistemic version and fails.

Failure cases are phase proof, not polish.

## 20. Suggested implementation order

This is an implementation order inside the spec, not a ticket decomposition.

1. Add stable epistemic IDs and typed proposition model with canonical serialization tests.
2. Add observation, belief, contradiction, and knowledge-context data structures in `tracewake-core`.
3. Extend content schema/serialization/validation for initial expectations and reject shortcut truth fields.
4. Add Tomas's source-backed expectation fixture data and Phase 2A fixture constructors.
5. Add epistemic event kinds/stream and projection application/rebuild path.
6. Add `check_container` action/effect through the existing action registry and pipeline.
7. Implement expectation contradiction for Tomas's strongbox check.
8. Add actor-known notebook projection and debug epistemics projection.
9. Extend TUI parser, app, renderers, and debug panels for notebook and epistemic debug commands.
10. Add `KnowledgePreconditionNotMet` and the minimal truthful-accusation probe/validation helper.
11. Add Elena sound-observation slice with uncertainty and no theft knowledge.
12. Add no-human scheduler/test-origin check path using the same pipeline.
13. Add replay/projection rebuild tests and unsupported version failures.
14. Add transcript/golden tests and ensure all existing Phase 1/1A gates still pass.
15. Update README/docs/spec ledger only after the implementation behavior is real.

## 21. Documentation updates required

Update documentation only after code/tests prove the behavior.

Required updates:

- `README.md`
  - Add `notebook`.
  - Add `debug epistemics`.
  - Add `debug beliefs <actor_id>`.
  - Add `debug observations <actor_id>`.
  - Add `check_container`/semantic action example.
  - Update the example session to show Tomas discovering absence through check/search, not open-container magic.

- `docs/4-specs/SPEC_LEDGER.md`
  - Add Spec 0004 entry after implementation lands.
  - Mark it as Phase 2A, not full Phase 2.
  - State explicitly what remains deferred.

- Fixture documentation/contracts
  - Add Phase 2A fixture contracts for required golden scenarios.
  - Preserve Phase 1 fixture contracts where applicable.

- `AGENTS.md` / `CLAUDE.md` only if needed
  - Update coding-agent guidance if new test command names or invariant checks are introduced.
  - Do not add product doctrine there that belongs in docs.

No documentation update may claim Phase 2 is complete unless all Phase 2 gates, not merely Phase 2A, are complete.

## 22. Final definition of done

Phase 2A is done when a reviewer can run the repository and verify all of this without reading implementation code:

1. `cargo run -p tracewake-tui` still starts the TUI.
2. The TUI can bind Mara.
3. Mara can move/take `coin_stack_01` using ordinary actions and stable semantic action IDs.
4. The TUI can bind Tomas.
5. Tomas's embodied view and notebook do not reveal Mara's action.
6. Tomas can intentionally check/search `strongbox_tomas`.
7. The check/search records an observation.
8. The observation contradicts Tomas's source-backed expectation.
9. Tomas receives a source-backed belief that `coin_stack_01` is missing from its expected location.
10. Tomas's notebook shows that belief, its source, confidence, acquisition tick, and contradiction summary.
11. Tomas's notebook does not name Mara as culprit.
12. Elena's sound observation, if present, remains low-confidence and non-culprit-bearing.
13. A truthful accusation/claim probe rejects for knowledge precondition when Tomas lacks actor-known support.
14. Debug epistemics shows ground truth, all relevant observations, all relevant beliefs, contradiction links, possession history, and truth/belief mismatch.
15. No-human epistemic check runs without controller/player references.
16. Replay rebuilds physical and epistemic projections.
17. Unsupported epistemic event/proposition/projection versions reject loudly.
18. Content validation rejects belief/source/proposition/shortcut failures.
19. Existing Phase 1/1A fixtures and TUI commands still pass.
20. All workspace format, lint, build, and test gates pass.

When this definition is satisfied, Tracewake will have its first executable proof that belief comes before truth. Only then should the project continue toward later Phase 2 slices or Phase 3 ordinary-life routines.
