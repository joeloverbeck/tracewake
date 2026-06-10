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
