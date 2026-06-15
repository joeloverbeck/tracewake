# Speech Acts, Language Surfaces, and LLM Boundary

## Status

Authoritative architecture contract.

## Purpose / core rule

Speech is structured simulation action over typed claims. Language is a surface. LLMs, if ever used, are optional render/parse/summarize aids behind validation and deterministic mocks. The core simulation must run, test, replay, and accept with LLMs disabled.

## Authority owned

This document owns speech-act structure, speaker/listener context boundaries, language rendering/parsing, testimony/lie/error handling, prompt packet limits, deterministic template requirements, and the optional LLM boundary.

## Authority denied

No language system may create authoritative truth, choose plans, repair quests, spawn clues, decide guilt, bypass validation, or become an agent brain.

## Contract

### Structured speech act

Speech is an ordinary proposal and event sequence:

```text
speaker trigger
 -> sealed speaker-known context
 -> speech intention/procedure decision
 -> structured speech proposal
 -> shared validation/event pipeline
 -> utterance event with typed claims and surface text
 -> listener perception event, if heard/read
 -> sealed listener-known interpretation context
 -> listener belief/memory/obligation/procedure update or rejection/doubt
```

The speaker can assert, ask, command, promise, report, accuse, deny, lie, joke, misremember, misunderstand, or stay silent. The listener can believe, doubt, ignore, mishear, record, report, contest, or act procedurally.

### Speech act minimum

```text
speech_act:
  speech_act_id
  speaker_id
  listener_ids_or_public_audience
  channel
  tick_or_window
  illocutionary_force
  typed_claims[]
  speaker_stance[]
  speaker_known_context_id
  surface_text_or_template_id
  visibility_scope
  validation_report_id
  source_event_ids
```

Language surface may be blank, templated, generated, translated, summarized, or player-entered. The typed speech act is authoritative; the prose is not.

### Speaker-known context

A speaker may speak only from claims, memories, records, rumors, roles, intentions, or lies available through modeled context. A lie is a structured act: the speaker asserts a proposition while holding a different stance or lacking support. A lie still cannot insert truth into listener knowledge unless it is spoken and perceived.

### Listener-known interpretation

Listeners do not receive raw speaker memory. They receive an utterance through a channel with audibility/visibility, trust, relationship, prior beliefs, language comprehension, social context, and possible distortion.

Listener updates must preserve:

- source speaker or artifact;
- channel;
- proposition;
- confidence/trust stance;
- whether the claim is hearsay;
- staleness;
- privacy/public scope;
- any contradiction with prior beliefs.

Temporal utterances are structured claims. Expressions such as yesterday, late,
due, before a market closure, or after a bell are interpreted through speaker
and listener holder-known temporal context, provenance, and ambiguity under
documents 03 and 06; the surface phrase is not itself temporal authority.

### Institutional speech

Reports, complaints, testimony, notices, orders, warrants, contracts, verdicts, sanctions, and role commands are speech/record acts with institutional authority only when issued by modeled roles through modeled procedures and resources. A town watch does not know a theft because a report exists; it knows that a report was made.

### LLM optional surface

LLMs may be used only behind hard boundaries:

Allowed future uses:

- render a structured speech act into prose;
- parse player prose into a candidate structured speech/action draft;
- summarize a holder-known notebook for display;
- produce stylistic variation for text that is not proof;
- help author content offline, followed by schema validation and deterministic fixtures.

Forbidden uses:

- LLM agent minds or planners;
- LLM validation of world truth;
- LLM facts inserted into state without modeled source;
- LLM quest repair, clue spawning, or pacing;
- LLM-only acceptance tests;
- live model output required for replay;
- prompt packets containing hidden truth outside the holder's context unless the packet is explicitly debug-only.

### Prompt packet boundary

If an LLM surface is used, its prompt packet must declare:

```text
prompt_packet:
  mode: embodied | debug | offline_authoring
  holder_known_context_id_or_none
  typed_claims_included
  hidden_truth_included: false for embodied
  allowed_output_schema
  deterministic_mock_key
  validation_required: true
```

Embodied packets may include only holder-known claims and permitted style instructions. Debug packets may include truth comparison only if output is marked non-diegetic and cannot feed simulation state.

### Determinism

Tests use deterministic templates or deterministic mocks. Replay records structured acts and optional surface references, not live model output. If generated text matters to an event, the accepted text is committed as payload with source and schema version.

## Required diagnostics / replay / TUI hooks

- Speech proposals and listener interpretation updates must have typed claims and source links.
- TUI may display surface text but should expose stable speech/action IDs for tests.
- Debug speech inspectors may show speaker known context, listener context, truth comparison, and hallucination/lie markers only as debug.
- LLM-disabled test runs must cover all acceptance-critical speech flows.

## Acceptance implications

The first proof does not require LLMs. Missing-property reports, accusations, notices, and institutional intake can use deterministic structured templates. Acceptance fails if a language surface can invent a fact, advance a procedure, or reveal hidden truth without modeled speech/record events.

## Anti-patterns

- “Ask the LLM what the actor should do.”
- “The user typed a command in natural language, so execute the parsed target directly.”
- “The report text says Mara stole it, so the institution knows Mara did it.”
- “The LLM summary says the clue is important, so show it as a lead.”
- “Prompt hidden truth but instruct the model not to reveal it.”
- “Use generated dialogue as the canonical claim.”

## Cross-document obligations

Speech acts consume holder-known contexts from document 03, create epistemic carriers under document 06, feed institution procedures in document 08, appear in TUI surfaces under document 10, and are accepted through document 13.
