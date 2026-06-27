# Epistemic View Models, Possession, and Debug Proof

## Status

Live execution doctrine for epistemic and TUI certification. Replaces the old Phase 2 execution document and related support sections.

## Authority boundary

This document owns execution acceptance for actor-known view models, possession parity, embodied affordances, notebooks, why-not output, and debug quarantine. It does not certify current implementation.

## Depends on

- `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md`
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
- `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md`
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`

## Core rule

The embodied TUI is a holder-known surface. Debug is a non-diegetic operator surface. The two may be adjacent in tooling; they must be separate in authority.

## Possession

Possession is a controller binding. It does not create a player character in the world.

A possession binding may:

- route human input to an ordinary actor;
- choose which actor-filtered view model is rendered;
- show non-diegetic controller/debug panels to the operator;
- unbind or rebind input when allowed by controller rules.

A possession binding may not:

- grant hidden truth;
- reset needs, memory, intentions, beliefs, routine progress, or obligations;
- grant special actions;
- alter validation rules;
- cause institutions or actors to recognize the possessed actor as special;
- make world state know that a human exists.

## Embodied view model

An embodied view model must be derived from the possessed actor's holder-known context and immediate perceptions. It may show:

- current actor-known place and sensory facts;
- known exits, containers, objects, people, records, and affordances;
- actor-legible need/status summaries;
- actor-known notebook entries;
- actor-visible why-not messages;
- stable semantic action identifiers for proposals the actor can consider.

It must not show hidden item locations, culprit truth, debug rejection details, true unseen route graph, true contents of closed/unknown containers, authoring-only flags, or player-only memory.

## Freshness parity across embodied surfaces

Possession, embodied TUI view models, notebooks, and no-human review surfaces
use the same freshness classifier as actor-known and institution-known
contexts in `04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md`.
Only current modeled perception, contact, or search may be `observed_now`;
older facts remain remembered, believed, stale, contradicted, hearsay,
record-derived, or unknown with source event, acquisition time, last-verified
time, and provenance preserved.

No possessed view, notebook, debug attachment, replay, display refresh, or
harness extraction is a fresher epistemic surface than autonomous actor
cognition. Possession and UI rendering may not restamp stale knowledge by
re-reading truth.

## Temporal Rendering and Time-Control Proof

Embodied temporal labels such as late, stale, closed, due, soon, recently, or
missed must cite modeled sources, not a readable clock, raw queue, replay
timestamp, or validator-only state. The source must be available to the viewed
actor through holder-known context, modeled perception, record, notice,
testimony, memory, or another source-backed channel.

Possession-parity evidence must show that possessing an actor does not refresh,
reveal, or reinterpret temporal facts beyond what that actor would know.
Possession, notebook display, transcript rendering, and embodied refreshes must
not convert stale or unknown temporal claims into current knowledge.

Embodied time-control proof must show that one-tick wait and advance-until
controls advance time through events, observations, waits, interruptions,
duration terminals, or missed-event summaries produced by modeled channels. Any
missed-event or interval summary must carry source and holder visibility, and
any stop reason shown in embodied play must be actor-known or source-bearing.
Debug exact time, raw queues, replay timestamps, and acceleration internals
remain non-diegetic and excluded from actor-known context. Per-surface temporal
rendering diagnostics are owned by
`10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`.

The accepted 0053 evidence makes the interval product a complete core result:
`LoadedWorldRuntime::from_bootstrap` constructs the runtime from
`ValidatedLoadedWorldBootstrap`, the shared world step captures final
perception, holder-known interval delta, stop reason, temporal verdict, and
replay-facing ancestry before returning the typed result to the TUI. The TUI
stores and renders that sealed interval product read-only through
`ContinuedRuntimeReceipt`; it does not call event-appending perception helpers,
rebuild holder-known context, construct `TypedActorKnownIntervalSummary`, or
convert debug step reports into embodied interval summaries. The read-only
boundary is witnessed by `crates/tracewake-tui/src/app.rs`,
`crates/tracewake-core/tests/holder_known_interval_projection.rs`,
`crates/tracewake-core/tests/salient_stop_actor_known.rs`, public
command-loop/TUI world-advance tests, `embodied_flow.rs`, and the external-crate
negative fixtures `external_crate_cannot_construct_actor_interval_summary`,
`external_crate_cannot_mutate_embodied_temporal_fields`, and
`external_crate_cannot_submit_debug_command_without_token`. Mutation sensitivity
for same-source observation replacement, subject-separated interval facts, and
the closed `food_source_fact_supersedes` family is recorded in
`archive/tickets/0052FOUCONFOU-010.md` and
`archive/tickets/0053FOUCONFIF-007.md`.

## Observation-time snapshot proof

For any actor-visible action, menu option, possession preflight, or embodied
view, evidence must show the holder, the modeled observation/bind/preflight/
perception boundary, the captured facts, their provenance and freshness, and
the absence of live-truth handles in the view-generation path. The proof is
capture sufficiency at formation time, not merely a final rendered-output check
that hidden facts are absent.

Wallhack negatives must fail when a datum is true in the world but not
known/perceived by the holder. Required negative classes include:

- true-but-unknown routes;
- true-but-unknown workplaces;
- true-but-unknown sleep affordances;
- true-but-unknown container contents;
- true-but-unknown item locations;
- true-but-unknown routine opportunities.

Possession-bind perception remains an owner decision. This document asserts
neither policy: a future owner decision may choose bounded bind-time perception
with modeled provenance/freshness, or no perception and no freshening. Either
policy must be proved with the same snapshot and parity discipline, and any
bind-time perception must be a modeled channel for the actor, never a human
knowledge transfer.

## Playable-capability parity proof

Every TUI-facing capability proof must include actor-known positive evidence
and actor-known negative evidence. Positive evidence shows the viewed actor had
the modeled knowledge, provenance, and freshness needed to see or attempt the
capability. Negative evidence shows hidden, stale, validator-only, or
debug-only truth did not become embodied affordance, notebook material, or
actor-visible why-not output.

Debug-quarantine evidence may explain why the validator accepted or rejected a
capability, but it cannot satisfy embodied parity. A parity proof must keep the
actor-known witness, the rendered actor-facing surface, and the debug witness
separate enough that a reviewer can tell which facts the actor could know and
which facts remained operator-only.

## Debug surface

Debug output may expose truth only if it is clearly non-diegetic and structurally quarantined. Debug output must not be reused as actor-visible prose, acceptance truth, planner input, institution input, or content authoring source.

All debug commands are gated by the current possessed actor's derived debug availability.
ControllerMode decision: debug availability requires ControllerMode::Debug.
ControllerMode::Embodied possession is ordinary embodied play and does not grant
debug availability; TUI callers that need operator debug surfaces must bind the
actor explicitly in debug mode. ControllerMode::Detached grants neither embodied
control nor debug.
The core command and debug-view boundary is also token-gated: public
debug/operator constructors require `DebugSessionAuthority`, so an external
client cannot submit `RuntimeCommand::run_no_human_day`, request
`RuntimeCommand::debug_view`, or construct exact debug view models without the
runtime-owned authority token.
The `debug run no-human-day` command is an operator-proof command, not an
embodied play verb: it may advance the loaded fixture through the no-human day
scheduler only after the debug gate is available, and its output remains a
non-diegetic debug panel. Embodied world-advancing controls use the ordinary
TUI/core boundary and must not reuse the debug no-human-day command or its
debug report as gameplay. Accepted embodied time controls require separate
one-tick, advance-until, actor-known interval-summary, anti-leak, and debug
quarantine evidence.

Debug artifacts should include:

- event log panels;
- item/location truth reports;
- projection rebuild reports;
- epistemic counts and holder beliefs;
- actor-known context traces;
- hidden-truth audit comparison rows;
- planner traces and stuck diagnostics;
- no-human metrics;
- rejection/failure reports with actor-visible and debug fields separated.

## Why-not output

A why-not message has two audiences:

| Audience | Allowed content |
|---|---|
| Actor-visible | What the actor can perceive, remember, infer, or be told; typed reason codes without hidden truth. |
| Debug/operator | Truth details, failed validation facts, responsible layer, hidden comparison rows, replay identifiers. |

The implementation must store these fields separately. A display string is not an authority boundary.

## Epistemic certification gate `EPI-CERT`

`EPI-CERT` is a phase-certification artifact label from the execution
sequence. It consumes canonical gate evidence; it is not a new canonical gate
code beyond `00_EXECUTION_INDEX_AND_AUTHORITY.md`.

`EPI-CERT` passes only when:

1. Observations, beliefs, contradictions, memories, records, and notebooks carry provenance.
2. Actor-known projections can be rebuilt from events and source artifacts.
3. Absence-as-evidence does not become culprit knowledge.
4. Possession parity fixtures prove human and non-human actors share rules.
5. View-model tests prove hidden facts are absent from embodied surfaces.
6. Debug panels are non-diegetic and cannot feed gameplay, planner, or actor affordances.
7. Why-not messages split actor-visible and debug fields.
8. Replayed projections match live projections.
9. Negative fixtures include closed-container, unknown-route, hidden-food, hidden-culprit, and debug-omniscience cases.
10. LLM-disabled operation remains complete.

## Audit implications

A future code audit must inspect view-model construction, notebook projection, possession binding, debug panel construction, rejection report construction, and tests that assert filtering. The audit must reject tests that parse debug strings and call the result actor knowledge.

## Failure examples

The following fail `EPI-CERT`:

- A possessed actor sees an action because truth says a hidden object exists.
- A notebook entry appears without a source observation, record, testimony, or prehistory event.
- Debug rejection details are copied into actor-visible text.
- Rebinding possession clears actor state.
- A test proves only that debug contains truth and then treats that as successful embodiment.
- A closed container's true contents affect embodied affordances before observation or search.
