# Ordinary Life, Needs, Routines, and No-Human Proof

## Status

Live execution doctrine for ordinary-life certification. Replaces the old Phase 3 execution document and absorbs durable lessons from archived specs `0005` through `0008`.

## Authority boundary

This document owns execution acceptance for needs, routines, intentions, no-human advancement, routine failure, and ordinary-life diagnostics. It does not certify the current implementation.

## Depends on

- `docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md`
- `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`
- `docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md`
- `docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md`

## Core rule

Ordinary life is not decorative NPC theater. It is the first proof that the simulation can continue without a human.

A hungry actor who cannot find known food must not teleport to true food, starve silently, or wait forever without cause. A tired actor must not sleep because a routine label says so unless a bed or sleep place is actor-known and validation accepts the proposal. A work routine must not start remote work without movement ancestry or a modeled reason.

## Needs

Needs are bounded pressures with event-sourced changes and actor-known visibility. They influence candidate goals. They do not name true targets.

A need may produce candidate families such as eat known food, search known surfaces, ask, travel to known place, sleep at known sleep place, continue work, wait with reason, or abandon an intention. It may not produce a direct primitive action using hidden truth.

Need accounting is single-charge-per-actor/per-need/per-tick across both passive
window deltas and action-emitted awake deltas. A modeled wait or other ordinary
action that advances a tick must leave ledger coverage that prevents the later
passive window from charging the same actor/need/tick again. Duration
`action_effect` charges are part of the same runtime assertion: their
`elapsed_ticks` payloads expand to per-tick ledger coverage before the
single-charge check runs.

## Single-owner derived-accounting seam

The single-owner seam for need and duration deltas is the
action-pipeline/ordinary-life boundary. It is the only layer permitted to emit
need deltas, duration ticks, work completion/failure accounting, passive-window
charges, or duration terminals.

The scheduler, routine planner, candidate generation, projection, replay, and
golden-output normalization may consume accounting evidence. They must not
independently charge, reconcile, synthesize, or normalize the same tick/window.
Every need delta, duration tick, work completion/failure, modeled wait, and
passive window is charged exactly once by the owning seam and then replayed or
reported from that event-backed ledger.

## Intentions

Intentions are durable commitments. They prevent jitter and make action history explainable.

Required lifecycle states:

- active;
- continued;
- suspended;
- interrupted;
- completed;
- failed;
- abandoned;
- replaced.

Every transition requires typed reason and ancestry. Possession must not reset intention state.

## Routines

Routines are defeasible HTN-like causal machinery. They are not scripts and not guaranteed arcs.

A routine template must state:

- holder-known applicability conditions;
- steps or method families;
- required known affordances or records;
- interruptors;
- failure modes;
- fallback/wait/stuck semantics;
- trace and diagnostic expectations.

A routine without failure modes is invalid.

## Routine Temporal Premises and Adaptation Proof

Each routine or social-rhythm proof must identify the temporal premise source
category it used. The proof is that a modeled channel supplied the premise, not
that a vocabulary token exists. Valid source categories include assignment,
memory, observation, public cue, record, testimony, institutional context, and
source-backed inference.

No-human scenarios must include routine behavior that succeeds from modeled
temporal premises and waits or fails when only ground-truth time would justify
action. A routine that would be correct under true schedule time must not be
selected when the actor lacks the source-backed premise. Scheduler awakenings
and elapsed-time accounting in
`05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` do not
count as routine-premise evidence.

Positive adaptation proof must show repeated modeled experience,
contradiction, interruption, notice, or changed routine outcome altering later
routine, method, or trust selection through holder-known memory or expectation
channels. Routine-failure diagnostics must classify missing knowledge, stale
knowledge, budget exhaustion, blocked affordance, and validation failure through
`10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`. This section
defines no temporal vocabulary, learning update rule, decay, trust-update
semantics, or threshold.

## No-human proof

The no-human proof must advance an authored fixture through ordinary actor transactions. It must show actors waking, needing, moving, eating, working, waiting, failing, replanning, sleeping, or becoming stuck through the same pipeline used by a possessed actor.

Accepted no-human proof evidence must include:

- no-human start and end markers;
- actor decision transaction traces;
- candidate goals and rejected candidates;
- selected methods and rejected methods;
- proposals and proposal ancestry;
- action validation outcomes;
- ordinary action events or modeled wait/failure events;
- need and intention changes;
- stuck diagnostics where behavior cannot progress;
- replay rebuild report;
- debug summaries that are non-diegetic.

Human-driven ticks are part of the same ordinary-life proof surface. A human
one-tick wait, continuation through an open sleep/work duration, or
advance-until control must preserve the same single charge per actor/need/tick,
passive suppression, duration-regime classification, open-duration completion,
and replay evidence required of no-human progression.

Human/no-human equivalence evidence must compare the shared loaded-world step
under held-equal authored state and inputs: the possessed actor's input slot
may differ, but other actors, due durations, passive accounting, and world
processes advance through the same owning seams. A proof that exercises only a
private possessed-actor tick or only the debug no-human runner does not satisfy
human-driven ordinary-life acceptance.

The current 0050 evidence names the executable basis for this comparison:
core derives loaded actor work and declared process work, then commits actor
transaction artifacts through the same `transact_world_one_tick` path for
human-origin and no-human-origin advancement. Acceptance rows must cite the
production mixed-schedule and adversarial witnesses in
`crates/tracewake-core/tests/world_step_coordinator.rs`,
`crates/tracewake-core/tests/generative_lock.rs`, and
`crates/tracewake-tui/tests/parity_adversarial.rs`, not the historical 0048
harness-injected population as current reachability proof.

## Canonical recovery resolution

The canonical `no_human_day_001` recovery resolution is intentionally recorded as
`fail_only_empty_food_source`. Mara's empty home food source must produce a typed
ordinary-life failure without consuming hidden fallback food, moving to Tomas's
food, or targeting Tomas's food through an unsupported shortcut. This is a
fixture decision, not an implicit relaxation of the no-human proof.

The success-recovery variant recommended by archived spec 0005 section 12 is
staged until Phase 3B routine/economy work makes public food services or
neighbor/public pantry access first-class actor-known affordances. At that
trigger, the canonical vocabulary must be re-evaluated by adding an explicit
supported recovery token and guard arm, or by recording a renewed staging
decision here. Silence is drift.

## Behavioral progress

Behavioral progress is one of:

- a committed ordinary action event;
- a committed duration start/completion/interruption event;
- a modeled wait with typed reason;
- a typed stuck/failure record that names blocker and responsible layer.

A pure `continue_routine` marker is not behavioral progress.

Cross-tick stuck detection must include:

- no-progress-past-expected-window: a routine window passes its expected progress boundary without a committed ordinary action, modeled wait, duration terminal event, or typed failure;
- repeated-idle: an actor repeatedly reaches idle/wait outcomes inside the same routine window without new behavioral progress.

Both categories must emit typed stuck diagnostics with responsible layer and blocker code, and no-human metrics may count them only as stuck/failure outcomes.

## Required ordinary-life adversarial fixtures

The certification suite must include these fixture families, regardless of exact filenames:

| Fixture family | Required proof |
|---|---|
| Integrated no-human day | Multiple actors progress without human input through ordinary transactions and replayable events. |
| Food unavailable | An actor with hunger cannot use hidden food truth and records fallback, search, wait, or stuck diagnostics. |
| Routine no-teleport | A work or sleep routine cannot begin at a remote target without movement ancestry or valid duration semantics. |
| Possession does not reset intention | Binding a controller does not reset need, routine, intention, memory, or actor-known context. |
| Hidden-truth planning | Hidden target truth exists in the fixture and is excluded from planning. |
| Planner trace | Selected and rejected candidates/methods are visible in debug without feeding actor input. |
| Routine blocker | Blocked routine records typed diagnostics rather than looping silently. |

## Certification gate `ORD-LIFE-CERT`

`ORD-LIFE-CERT` is a phase-certification artifact label from the execution
sequence. It consumes canonical gate evidence; it is not a new canonical gate
code beyond `00_EXECUTION_INDEX_AND_AUTHORITY.md`.

`ORD-LIFE-CERT` passes only when:

1. Needs, intentions, routines, and stuck state are event-sourced or replay-reconstructable.
2. Candidate generation uses actor-known inputs only.
3. Method selection and local planning cite actor-known provenance.
4. Scheduler cannot dispatch ordinary actions directly from needs or routines.
5. Action validation rejects forged/stale proposal parameters.
6. No-human metrics count only real progress, modeled waits, or typed stuck/failure outcomes.
7. Debug output can compare actor-known input against hidden truth without contaminating decisions.
8. Replay rebuilds ordinary-life metrics and diagnostics.
9. Fixture failures identify the responsible layer.
10. Phase 4 remains blocked until `ORD-LIFE-CERT` and the preceding certification gates pass.

## Explicit non-goals under this gate

This gate does not require personality simulation, complex social drama, full economy, institutions, gossip, LLM conversation, travel, regional scale, or wrong-suspicion procedures. Those systems consume ordinary-life correctness; they do not replace it.
