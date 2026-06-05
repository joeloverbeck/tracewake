# Phase Ladder and Acceptance Gates

## Core rule

Execution is a strict ladder:

```text
Phase 0 -> Phase 1 -> Phase 2 -> Phase 3 -> Phase 4 -> Second Proof
```

A later feature cannot compensate for an earlier failure. Notices cannot hide shallow records. Travel cannot hide weak routines. Companion recruitment cannot hide missing actor autonomy. A polished TUI cannot hide a weak action/event/replay spine. A debug command cannot stand in for embodied play.

## Phase identities

| Phase | Identity | Must prove | Unlocks |
|---|---|---|---|
| Phase 0 | Paper ontology and fixture contracts | The missing-property chain can be manually traced as actions, events, traces, observations, beliefs, speech acts, reports, records, suspicion, replay, and debug views without scripts. | Phase 1 kernel work. |
| Phase 1 | Runnable kernel, TUI/view-model harness, event log, and replay | Local movement, doors, containers, item movement, action rejection, event commit, replay rebuild, debug explanation, and no-human scheduler advance. | Phase 2 epistemics. |
| Phase 2 | Epistemics, actor-known view models, and possession parity | Observation is not interpretation; beliefs have holder/source/confidence; absence requires expectation; embodied mode hides truth; possession transfers no knowledge. | Phase 3 ordinary life. |
| Phase 3 | Needs, routines, work, and no-human life | Hunger, fatigue, safety, sleep, food, work, intentions, routine failure, interruptions, and at least one no-human ordinary day. | Phase 4 institutions. |
| Phase 4 | Institutions, records, and wrong suspicion | Households, local authority, report intake, incident ledger, procedure, institutional non-omniscience, and source-backed wrong suspicion. | Second proof. |
| Second proof | Notices, travel, and regional expansion | Public artifacts, stale leads, route risk, companion recruitment, proof/payment, and regional boundary processes reuse first-proof machinery. | Honest expansion outward. |

## Universal phase shape

Every phase document and later implementation spec must state:

- purpose;
- entry requirements;
- deliverables;
- explicit non-goals;
- TUI/view-model gate;
- no-human gate;
- deterministic replay gate;
- test gate;
- data/fixture gate;
- debug/inspection gate;
- phase exit criteria;
- forbidden shortcuts;
- modeled failure cases;
- what must not start until the phase passes.

No runnable phase may omit TUI/view-model acceptance. No runnable phase may omit no-human execution. No accepted mechanic may bypass the action/event pipeline.

## Acceptance formula

A runnable phase is not accepted until this is true:

```text
mechanic runs
+ embodied TUI or actor-filtered view-model reaches it
+ no-human variant runs where phase scope allows
+ deterministic replay rebuilds it
+ tests cover success and failure
+ fixtures/data validate it
+ debug explains it
+ no script/player/LLM/ground-truth leakage exists
```

Manual demos are useful. They are never phase exit evidence by themselves.

## Phase 0 acceptance gates

Purpose: prevent bad ontology before Rust kernel work begins.

Deliverables:

- primitive action vocabulary;
- primitive event vocabulary;
- primitive proposition vocabulary;
- primitive trace vocabulary;
- primitive speech-act vocabulary;
- first-proof actor roster;
- village map and object inventory;
- manual chains for missing property, ordinary workday, report/record, wrong suspicion, possession parity, and no-human daily life;
- embodied and debug view-model sketches;
- golden scenario sketches;
- no-scripting review.

Exit criteria:

```text
A reviewer can manually trace the first proof from cause through action, event, trace,
observation, belief, speech, report, record, suspicion, replay, TUI visibility, and debug
explanation without requiring a quest object, hidden culprit script, player identity, or LLM authority.
```

Do not start Phase 1 until Phase 0 names enough vocabulary and fixture chains to prevent architectural mistakes. Phase 0 is not an academic thesis; it is an execution contract.

## Phase 1 acceptance gates

Purpose: build the physical/event/action/TUI spine before belief depth.

Minimum exit evidence:

- minimal village fixture loads with stable IDs;
- TUI or view-model harness can attach to an actor, move locally, inspect place/object/container, open/close door/container, take/place item, wait, and request why-not;
- actions pass through shared proposal/validation/event pipeline;
- meaningful changes append versioned events;
- replay rebuilds door state, container state, item location, and event order;
- debug explains why an item is where it is;
- no-human scheduler advance runs with no controller bound and no player references;
- validation rejects player/quest/direct-script shortcuts.

Do not start full epistemics, actor-known notebooks, needs/routines, institutions, suspicion, road travel, or LLM surfaces before Phase 1 passes.

## Phase 2 acceptance gates

Purpose: prove belief-before-truth and possession parity.

Minimum exit evidence:

- observation events exist for direct perception, search/touch, simple sound, and absence where expectation exists;
- beliefs require holder, proposition, stance, confidence, source, acquisition time, and channel where relevant;
- expectation contradiction works and does not trigger for actors with no expectation;
- embodied view models contain only actor-known/perceived/source-backed facts;
- debug view models reveal truth only as non-diegetic inspection;
- possession switch changes input binding only;
- a player can act as Mara, switch to Tomas, and Tomas cannot truthfully accuse from human memory;
- no-human perception/belief updates run for actors without a controller;
- replay rebuilds observations, belief updates, contradictions, and actor-known snapshots.

Do not start daily no-human proof, institutional report intake, wrong suspicion, notices, travel, or companions before Phase 2 passes.

## Phase 3 acceptance gates

Purpose: make ordinary life real enough that disruption matters.

Minimum exit evidence:

- hunger, fatigue, and safety affect behavior;
- sleep, food, work, wait, and continue-routine actions run through TUI/view models;
- routines are defeasible intentions with preconditions, duration, failure modes, interruptions, and planner/debug traces;
- at least one full simulated day runs with no human controller bound;
- agents wake, eat, work, rest, sleep, move, speak minimally, and respond to modeled interruptions;
- no unpossessed actor freezes because no human is attached;
- replay rebuilds needs, scheduled actions, routine starts/completions/failures, and salient planner traces;
- stuck actors and planner failures produce diagnostic events.

Do not start final report/record proof, institutional wrong suspicion, notice lifecycle as product feature, route threat, travel, or companions before Phase 3 passes.

## Phase 4 acceptance gates

Purpose: complete the first proof through fallible social machinery.

Minimum exit evidence:

- households affect access, storage, food, privacy, expectations, and ordinary movement;
- one local authority exists with clerk/reeve/guard roles, office hours, limited resources, jurisdiction, and procedure;
- report is a structured speech act through the action pipeline;
- records are artifacts with author, source claims, provenance, custody, access, status, and contradiction/amendment path;
- norms distinguish violation, detection, suspicion, report, record, proof, and sanction;
- suspicion uses actor/institutional knowledge, never ground truth;
- wrong suspicion arises for legible source-backed reasons;
- no-human missing-property discovery/report/record/suspicion can occur under modeled conditions;
- replay rebuilds institutional beliefs, records, suspicion scores, and procedure state;
- debug explains truth, belief, traces, records, suspicion, and why the institution is wrong.

Phase 4 completes the first proof. Only after it exits may the second proof begin.

## Second-proof start gate

Second-proof work may begin only when:

- all Phase 4 exit criteria pass;
- first-proof golden fixtures are automated;
- no-human first-proof run passes;
- replay rebuilds first-proof event, belief, record, and TUI/debug projections;
- actor-known/debug view separation is regression-tested;
- institution non-omniscience is proven;
- no player-only verbs exist;
- data validation rejects quest/player/outcome-chain fields.

Second-proof features cannot weaken first-proof invariants. A notice, route, companion, proof, or payment is accepted only if it reuses the same action, belief, record, custody, TUI, no-human, and replay machinery.

## One-line implementation order

```text
paper trace the missing-property village
 -> build physical action/event/TUI spine
 -> add belief/view/possession parity
 -> add needs/routines/no-human day
 -> add local authority/report/record/wrong suspicion
 -> only then build notice/travel/regional proof
```
