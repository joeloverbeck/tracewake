# Testing, Validation, and Debugging

## Core claim

Emergent simulation without forensic testing becomes superstition.

Tests must ask:

- did the event have causes;
- did the right agents know and not know;
- did institutions act from records rather than omniscience;
- did UI reveal belief rather than truth;
- did the same action pipeline serve player and NPCs;
- can replay explain the outcome;
- did LLM output stay bounded;
- was the mechanic playable through the TUI?

## Test categories

### Unit tests

Test action preconditions, event creation, component mutation, ownership/possession, locks, traces, belief update, confidence decay, norm classification, speech-act validation, notice posting, view-model filtering.

### Integration tests

Test theft discovery, report intake, bounty lifecycle, notice lifecycle, stale lead travel, wrong suspect, possession switch, routine interruption, companion recruitment, route travel, no-player daily village.

### Property tests

Required properties:

- no item has two possessors;
- every possession change has event;
- no belief lacks source;
- no important record lacks author/provenance;
- no institutional case lacks report/observation/inference/procedure;
- no notice lacks author/issuer/structured claims/posting event;
- no guard knows unseen crime unless informed, observing, inferring, or reading;
- no player-only world action exists in core registry;
- no LLM output commits unvalidated facts;
- no embodied view model contains debug-only truth;
- every phase mechanic has TUI reachability test.

### Fuzz tests

Run small villages across seeds. Check:

- planner loops;
- reservation deadlocks;
- impossible paths;
- missing event order;
- stuck actors without cause;
- runaway rumors;
- belief-store explosion;
- stale info correction bugs;
- LOD promotion/demotion ancestry;
- player-conditioned event anomalies.

### Golden scenario tests

Use deterministic seeds.

Example:

```text
Seed: strongbox_001
Expected:
- Mara steals coins.
- Tomas does not know until checking.
- Elena has low-confidence noise belief.
- Report creates case record.
- Guard suspicion remains below arrest threshold.
- Player switching to Tomas does not transfer Mara knowledge.
```

Verify both occurrence and non-occurrence.

## TUI acceptance tests

Every runnable phase needs TUI-level or view-model-level tests.

Examples:

- action menu lists only actor-available actions;
- why-not reasons cite correct failed precondition;
- embodied mode hides ground truth;
- debug mode reveals causal graph;
- lead notebook labels uncertainty and source;
- notice board reflects physical notices;
- possession switch changes input binding only;
- conversation UI commits structured speech acts;
- wait/sleep controls run simulation and stop on actor-perceived interruption.

## Belief tests

Assert:

- holder;
- proposition;
- confidence range;
- source;
- acquisition time;
- channel;
- contradiction links;
- stale/decay state;
- actor visibility in TUI.

## Institution tests

Assert:

- report receiver;
- office accessibility or exception;
- ledger entry;
- priority from known facts;
- guard assignment through procedure;
- notice authorization;
- funds reserved;
- proof required;
- payment/refusal cause.

Forbidden pass:

```text
bounty exists because incident exists
```

Required pass:

```text
bounty exists because institution learned, assessed, reserved funds,
wrote notice, and posted it
```

## Epistemic leakage tests

Scenarios:

- player knows truth, actor does not;
- accusation unavailable without actor-known support;
- markers show believed location, not true location;
- dialogue cannot reveal hidden truth;
- institution does not read player notebook;
- possession history does not alter beliefs;
- debug view state never appears in embodied view.

## LLM validation tests

Use deterministic mocks.

Assert:

- generated text maps to structured speech acts;
- unsupported propositions do not commit;
- lies require motive/social act;
- summaries preserve uncertainty;
- core works with LLM disabled;
- prompts contain only actor-allowed information;
- output cannot create records, clues, guilt, or quests.

## Replay

Replay reconstructs:

- state at tick;
- event sequence;
- random draws;
- action proposals;
- precondition checks;
- belief updates;
- planner decisions;
- institution decisions;
- LOD promotions/demotions;
- speech-act commits;
- TUI view model snapshots for important acceptance tests.

Debug mode should answer:

- why did Tomas report theft;
- why did Elias suspect Oren;
- why did notice stay posted;
- why did Mara go to tavern;
- why could Tomas not accuse Mara;
- why did Oren refuse recruitment;
- why did a routine fail;
- why did the player-controlled body lack an option.

## Metrics

Track:

- significant events;
- routine events;
- causal coverage;
- no-cause events;
- belief updates;
- contradictions;
- stale beliefs;
- reports;
- notices;
- rumors;
- lies;
- wrong accusations;
- investigations;
- recruitment;
- travel;
- LOD changes;
- planner time;
- stuck actors;
- player-conditioned event rate;
- LLM validation failures;
- TUI action coverage.

## No-player tests

Run village without human input. Validate:

- routines;
- trade;
- hunger/fatigue/safety handling;
- conversations;
- rumors;
- reports/notices when causal;
- wrong beliefs;
- stale records;
- ordinary economy flows;
- no protagonist gravity.

## Validated vertical slice

The vertical slice is valid when:

- required scenarios pass deterministically;
- randomized seeds produce no invariant violations;
- testers can explain major outcomes;
- wrong beliefs and stale info are useful;
- no core content depends on player identity;
- TUI supports normal embodied play;
- debug mode can inspect causal chains;
- replay works for major consequences.

## Anti-patterns

- passing headless tests while TUI cannot play feature;
- testing only success paths;
- belief test checks truth rather than holder/source;
- LLM live calls in golden tests;
- debug assertions relying on player identity;
- institution tests reading ground truth;
- replay that cannot explain why;
- metrics ignored until scale is broken.
