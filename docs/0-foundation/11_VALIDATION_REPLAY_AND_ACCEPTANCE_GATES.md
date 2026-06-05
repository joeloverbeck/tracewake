# Validation, Replay, and Acceptance Gates

## Core claim

Tracewake features are not accepted because they are plausible in prose.

They are accepted when they run, replay, explain themselves, preserve actor knowledge, avoid player privilege, remain playable through the TUI, and still work with no human and no LLM.

The validation doctrine must be harsh because the seductive wrong implementation is easy: hidden scripts, omniscient UI, player-only verbs, current-state-only simulation, LLM fact creation, and large opaque worlds.

## Universal feature questions

Every feature must answer:

```text
What caused it?
Who knows it?
How can they be wrong?
What traces exist?
What institution, household, norm, or record cares?
Can an NPC do the same kind of thing?
Can it be played and inspected through the TUI?
Does it leak player privilege, ground truth, genre assumptions, scripting, or LLM authority?
Can it run in a no-human simulation?
Can debug mode explain it?
Can it replay deterministically where meaningful?
```

If a feature cannot answer these questions, it is not foundation-compatible.

## Runnable phase gates

Every runnable phase must include gates for:

- no-human simulation;
- TUI playability;
- actor-knowledge filtering;
- possession parity where relevant;
- no player privilege;
- no hidden ground-truth leakage;
- deterministic replay;
- causal ancestry inspection;
- event and trace creation;
- belief provenance;
- institutional fallibility where relevant;
- no-script compliance;
- LLM-disabled operation;
- automated regression tests.

A phase may have additional gates. It may not remove these once relevant.

## No-human simulation gate

Every runnable phase must be able to run with no human controller bound.

The no-human simulation should verify, as relevant to phase scope:

- agents satisfy needs;
- routines proceed;
- work happens or fails;
- sleep and hunger matter;
- travel consumes time;
- property remains in custody unless moved;
- containers and access rules matter;
- speech acts propagate information;
- beliefs change only through modeled channels;
- reports and records can be created;
- institutions can act or fail;
- stale information remains stale until updated;
- events, traces, and failures are logged;
- no system waits for the player.

The first serious village must run for days without human input.

## TUI and view-model gate

Every runnable phase must include automated TUI or view-model acceptance.

The gate must show:

- the mechanic is reachable through embodied play;
- embodied view models are actor-filtered;
- hidden truth is absent in embodied mode;
- why-not explanations work;
- committed actions pass through ordinary validation;
- debug mode can reveal the full causal truth;
- the TUI does not implement simulation rules;
- the TUI does not create separate quest state;
- future clients can reuse the same view-model boundary.

Terminal-level tests are ideal when stable. View-model tests are acceptable when terminal layout is still changing.

## Possession parity gate

Possession is a foundational debug proof.

Tests must verify:

- `HumanController -> ActorId` binding changes only input;
- previous body continues as ordinary agent;
- beliefs stay with bodies;
- intentions stay with bodies;
- possessions and custody stay in world state;
- obligations and relationships stay with actors/institutions;
- human possession history does not become world fact;
- no action gains special authority because the human controls the body;
- possessing another actor does not reveal truth in embodied mode except through that actor's knowledge;
- debug inspector can reveal knowledge mismatch.

A player-only action path is a failure.

## Actor-knowledge leakage gate

Embodied mode must be tested against hidden truth.

Useful tests:

- actor cannot accuse from a fact learned only by the human while possessing another body;
- actor cannot report a culprit without modeled observation, testimony, inference, or record;
- actor cannot navigate to hidden evidence using an objective marker;
- actor notebook contains only source-bound actor-known leads;
- human/debug notes cannot satisfy action preconditions;
- debug inspector truth is visibly non-diegetic;
- institution does not recognize completion without evidence/procedure;
- stale records do not auto-correct when inspected by the human.

Leakage failures are high-severity foundation defects.

## Event and trace gate

Meaningful changes must produce events.

Tests should verify:

- state-changing actions emit events;
- failed meaningful attempts emit failure events;
- absence can become evidence when expectation exists;
- trace creation is eventful;
- trace removal is eventful;
- mental, social, institutional, economic, environmental, spatial, procedural, and regional events are represented where relevant;
- events retain cause links;
- events are queryable in debug.

A current-state-only mutation is not acceptable for meaningful changes.

## Replay and randomness gate

Tracewake must support deterministic replay of meaningful outcomes.

Tests should verify:

- same seed and inputs produce same meaningful event sequence;
- random draws that affect meaningful outcomes are logged or reproducible;
- no nondeterministic UI ordering changes authoritative outcomes;
- time stepping is deterministic under the same schedule policy;
- snapshots restore to equivalent replay state;
- compaction preserves significant causal ancestry;
- debug tools can compare runs or local event chains.

Perfect bitwise determinism across all future platforms may be an architecture issue. Replay trust for meaningful simulation outcomes is a foundation requirement.

## Causal ancestry inspection gate

Debug mode must answer why.

For a meaningful event, inspection should be able to show:

- immediate event;
- direct cause event(s);
- relevant state preconditions;
- actor intention or procedure;
- beliefs used;
- needs, motives, roles, or obligations involved;
- resources and access constraints;
- random draws if any;
- traces created or removed;
- beliefs/records changed;
- later events enabled.

A surprising event that cannot explain itself is not Tracewake-quality.

## Belief provenance gate

Belief-changing features must verify:

- holder;
- proposition;
- confidence;
- source;
- channel;
- acquisition time;
- believed event time if available;
- staleness;
- contradiction links;
- memory/provenance preservation;
- no telepathy;
- no automatic stale correction;
- no omniscient institution beliefs.

Lies, rumors, testimony, inference, reading, perception, instruction, memory, and institutional notification should be distinct channels.

## Institutional fallibility gate

Institution features must prove that institutions are social machines, not omniscient scripts.

Tests should include relevant failures:

- report refused;
- report accepted with partial information;
- record created with uncertain claim;
- record becomes stale;
- clerk absent or office closed;
- funds insufficient;
- reward delayed or refused;
- wrong suspicion arises from legible causes;
- jurisdiction blocks action;
- bias or credibility affects procedure;
- proof requirement not satisfied;
- record lost, misfiled, damaged, forged, or contradicted.

A perfect always-correct authority is an anti-pattern.

## No-script gate

Authored content must be tested against scripting failure modes.

Reject features that contain:

- player-conditioned event injection;
- quest state as primary ontology;
- guaranteed target existence;
- guaranteed reward payment;
- world waiting for player;
- automatic completion recognition;
- objective markers to ground truth;
- drama pacing mutation;
- LLM fact creation;
- hidden scripted red herrings or reveals.

Scenario seeds and HTN procedures are acceptable only when they define causal possibility, failure, interruption, and NPC use.

## LLM-disabled gate

Core mechanics must work with LLMs disabled.

Tests should use deterministic templates or mocks for language surfaces.

Validation must prove:

- structured speech acts work without LLMs;
- records and notices render without LLMs;
- agents plan without LLMs;
- facts are not created by prose;
- LLM parse/render failures do not corrupt authoritative state;
- disabling LLMs does not remove core gameplay.

LLMs are optional language surfaces. They are not foundation dependencies.

## Regression seeds

The project should maintain canonical deterministic seeds for regression.

Required early seeds should include:

```text
ordinary_day_no_human
container_and_access
household_storage
missing_property_discovery
uncertain_witness
lie_and_rumor
partial_report_to_clerk
wrong_suspicion
stale_notice
debug_possession_no_leak
replay_same_seed
llm_disabled_language_surface
```

These are not per-task templates. They are acceptance scenarios for protecting project identity.

## Definition of done

A runnable feature is done only when it is:

```text
caused
agent-possible
eventful
trace-aware
epistemically bounded
TUI-playable
debug-inspectable
no-human runnable
replay-safe
LLM-independent
non-scripted
regression-tested
```

Anything less is a prototype note, not a foundation-compatible feature.
