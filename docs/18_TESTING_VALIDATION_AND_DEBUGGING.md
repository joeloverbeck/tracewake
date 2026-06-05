# Testing, Validation, and Debugging

## Core claim

Emergent simulation without forensic testing becomes superstition.

Tests must ask: did the event have causes; did the right agents know and not know; did institutions act from records rather than omniscience; did UI reveal belief rather than truth; can replay explain outcome; did LLM output stay bounded.

## Test categories

### Unit tests

Action preconditions, event creation, component mutation, ownership/possession, locks, traces, belief update, confidence decay, norm classification, speech act validation, notice posting.

### Integration tests

Theft discovery, report intake, bounty lifecycle, stale notice expedition, wrong suspect, possession switch, routine interruption, companion recruitment, route travel.

### Property tests

No item has two possessors. Every possession change has event. No belief lacks source. No institutional case lacks report/observation/inference. No notice lacks author/posting event. No guard knows unseen crime unless informed. No player-only action exists in core registry. No LLM output commits unvalidated facts.

### Fuzz tests

Run small villages across seeds. Check planner loops, reservation deadlocks, belief-store explosions, impossible paths, missing event order, stuck actors without cause, and runaway rumors.

### Golden scenario tests

Use deterministic seeds. Verify both occurrence and non-occurrence.

```text
Seed: strongbox_001
Expected:
- Mara steals coins.
- Tomas does not know until checking.
- Elena has low-confidence noise belief.
- Report creates case record.
- Guard suspicion remains below arrest threshold.
```

## Belief tests

Assert holder, proposition, confidence range, source, acquisition time, channel, and contradiction links.

## Institution tests

Assert report receiver, ledger entry, office open or exception, priority from known facts, guard assignment through procedure, notice authorization, funds reserved, proof required.

Forbidden pass: bounty exists because incident exists. Required pass: bounty exists because institution learned, assessed, reserved funds, wrote notice, and posted it.

## Epistemic leakage tests

Player knows truth; actor does not. Accusations remain unavailable, markers show believed not true location, dialogue cannot reveal hidden truth, institution does not read player notebook, possession history does not alter beliefs.

## TUI tests

Action lists obey actor knowledge, why-not reasons cite correct preconditions, embodied mode hides truth, debug mode reveals causal graph, lead notebook labels uncertainty, notice board reflects physical notices, terminal renderer consumes view models.

## LLM validation tests

Use deterministic mocks. Generated text maps to structured speech acts. Unsupported propositions do not commit. Lies require motive/permission. Summaries preserve uncertainty. Core works with LLM disabled.

## Replay

Replay reconstructs state at tick, event sequence, random draws, action proposals, preconditions, belief updates, planner decisions, institution decisions, LOD promotions, and speech act commits.

Debug mode should answer: why did Tomas report theft; why did Elias suspect Oren; why did notice stay posted; why did Mara go to tavern; why could Tomas not accuse Mara; why did Oren refuse recruitment.

## Metrics

Track significant events, routine events, causal coverage, no-cause events, belief updates, contradictions, stale beliefs, reports, notices, rumors, lies, wrong accusations, investigations, recruitment, travel, LOD changes, planner time, stuck actors, player-conditioned event rate, and LLM validation failures.

## No-player tests

Run village without human input. Validate routines, trade, rumors, reports, notices, wrong beliefs, stale records, economy flows, and no protagonist gravity.

## Validated vertical slice

Required scenarios pass deterministically; randomized seeds produce no invariant violations; testers explain major outcomes; wrong beliefs and stale info are useful; no core content depends on player identity; TUI supports normal embodied play.
