# Testing, Validation, and Debugging

## Core claim

Emergent simulation without forensic testing becomes superstition.

The project needs tests that ask not merely “did it run?” but:

- Did the event have causes?
- Did the right agents know and not know?
- Did the institution act from records rather than omniscience?
- Did the UI reveal belief rather than truth?
- Can we replay and explain the outcome?

## Test categories

### Unit tests

For primitive systems:

- action preconditions;
- event creation;
- component mutation;
- ownership/possession;
- lock/open/close;
- trace creation;
- belief update;
- confidence decay;
- norm classification;
- notice posting.

### Integration tests

For causal chains:

- theft discovery;
- report intake;
- bounty lifecycle;
- stale information;
- wrong suspect;
- possession switch;
- routine interruption.

### Property tests

For invariants across random worlds:

- no item has two possessors;
- every possession change has event;
- no belief without source;
- no institutional case without report/observation/inference;
- no public notice without author/posting event;
- no guard knows unseen crime unless informed;
- no player-only action in core action registry.

### Fuzz tests

Run many small villages with random seeds and check:

- no infinite planner loops;
- no deadlock on shared resources;
- no exploding belief stores;
- no impossible path reservations;
- no event without timestamp/order;
- no actor stuck permanently unless cause is legible.

### Golden scenario tests

Use deterministic seeds and expected causal chains.

Example:

```text
Seed: strongbox_001
Expected:
- Mara steals coins.
- Tomas does not know until he checks.
- Elena has low-confidence noise belief.
- Report creates case record.
- Guard suspicion remains below arrest threshold.
```

Golden tests should verify both occurrence and non-occurrence.

## Validating beliefs

Belief tests should assert:

- holder;
- proposition;
- confidence range;
- source;
- acquisition time;
- whether belief is direct, hearsay, rumor, record, or inference;
- contradiction links.

Example assertion:

```pseudo
assert not guard.believes(MaraStoleCoins)
assert guard.believes(PossibleTheft(item_coin_stack), source=ReportByTomas)
assert elena.believes(HeardNoise(room_bedroom), confidence < 0.5)
```

## Validating institutions

Institution tests should assert:

- report received by actor;
- ledger entry created;
- office was open or special exception recorded;
- case priority computed from known facts;
- guard assigned through procedure;
- notice authorized and written;
- funds reserved for bounty;
- proof required for payment.

Forbidden test pass:

```text
Bounty exists because incident exists.
```

Required pass:

```text
Bounty exists because institution learned incident, assessed threat, chose bounty, reserved funds, and posted notice.
```

## Replay

Replay is mandatory.

A replay should reconstruct:

- state at tick;
- event sequence;
- random draws if debug build;
- action proposals;
- precondition outcomes;
- belief updates;
- planner decisions;
- institution decisions.

Debug mode should answer:

```text
Why did Tomas report theft?
Why did Elias suspect Oren?
Why did the notice stay posted after bandits died?
Why did Mara go to tavern instead of fleeing?
Why could the player not accuse Mara as Tomas?
```

## Metrics dashboard

Track per simulation:

- total significant events;
- routine events;
- belief updates;
- contradictions;
- stale beliefs;
- reports;
- notices;
- rumors;
- lies;
- wrong accusations;
- investigations opened;
- investigations closed;
- average planner time;
- max planner time;
- stuck actors;
- no-cause events;
- player-conditioned events;
- LOD promotions.

## Causal coverage metric

Define:

```text
causal_coverage = significant_events_with_causal_parent / significant_events
```

Target for prototype: above 0.95.

Some exogenous events may have root causes such as weather or random world process. They still need explicit cause type.

## Epistemic leakage tests

Run scenarios where the player knows something but current actor does not.

Assertions:

- unavailable accusation options remain unavailable;
- markers show believed location, not true location;
- dialogue does not reveal hidden truth;
- institutions do not update from player notebook;
- possession history does not alter actor beliefs unless diegetic possession is enabled.

## No-player simulation tests

Run the village without human input.

Validate:

- routines happen;
- incidents can occur;
- reports can happen;
- notices can be posted;
- wrong beliefs can form;
- stale records can exist;
- economy flows continue.

If the world is inert without the player, protagonist gravity has infected the design.

## Playtest protocol

Ask testers to play with these goals:

1. Watch for a day and explain one event chain.
2. Steal something and switch bodies.
3. Try to frame someone.
4. Pursue a notice that may be stale.
5. Ask “why did this happen?” five times.
6. Identify one thing the actor knows but player does not, and one thing player knows but actor does not.

Collect:

- confusion points;
- moments of causal delight;
- moments of perceived cheating;
- hidden omniscience bugs;
- UI uncertainty failures;
- boring periods with no readable artifacts.

## LLM validation

If LLMs are used, test:

- generated text contains only structured claims or marked speculation;
- no new entity/fact appears unless validated by content pipeline;
- lies are represented as speech acts with speaker belief mismatch;
- summaries preserve uncertainty;
- repeated generation does not change authoritative state.

A generated notice can vary in wording. Its structured claims cannot drift.

## Bug signatures

### Omniscient guard bug

Guard acts on truth without information path.

### Quest relapse bug

Task appears without issuer, record, or artifact.

### Schedule tyranny bug

Agent follows routine despite impossible or absurd conditions.

### Utility jitter bug

Agent changes goal too often and appears insane.

### Evidence teleport bug

Trace or proof appears in inventory/record without event chain.

### Player gravity bug

Interesting events cluster around player without modeled reason.

### Stale cleanup bug

System auto-corrects outdated information without information flow.

## Definition of validated vertical slice

The slice is validated when:

- every required scenario passes deterministically under at least one seed;
- randomized seeds produce no invariant violations over long runs;
- testers can explain major outcomes using UI/debug tools;
- wrong beliefs occur and remain legible;
- stale information occurs and remains useful;
- no core content depends on player identity.
