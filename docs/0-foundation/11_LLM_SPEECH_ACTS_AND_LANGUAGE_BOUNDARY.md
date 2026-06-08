# LLM, Speech Acts, and Language Boundary

## Core claim

Language is a surface over structured simulation state. It is not simulation authority.

Tracewake must support eventual rich conversation with ordinary agents, including future companions, recruitment, bargaining, travel talk, rumor, confession, argument, and adventure conversation. It must do this without letting language invent reality.

The authoritative layer is structured speech acts and typed claims. LLMs, if used, are optional rendering/parsing aids behind validation.

## V1 language model

V1 uses structured speech-act menus and deterministic templates.

Required early speech-act vocabulary includes staged versions of:

- greet;
- ask;
- answer;
- tell;
- report;
- gossip;
- refuse;
- lie;
- accuse/suspect;
- deny;
- promise;
- command/instruct where role permits;
- testify/simple statement;
- withhold/deflect.

Additional future acts may include confess, threaten, bargain, recruit, warn, apologize, joke, negotiate, command group, swear oath, challenge, comfort, teach, interrogate, and repair clarification. They remain structured acts.

## Speech as action

Speech acts pass through the ordinary action pipeline when they can affect the world.

A speech act validates:

- speaker can communicate;
- listener/audience/channel exists or is reachable;
- speaker has source belief, question, intention to speculate, intention to lie, role authority, or willingness to violate a norm;
- actor-knowledge preconditions are satisfied;
- privacy/overhearing conditions;
- time cost and interruption risk;
- social permission or willingness to violate;
- institutional role/procedure requirements;
- listener perception and interpretation.

Speech can fail or partially fail: listener absent, listener refuses attention, office closed, language mismatch, fear blocks report, speaker withholds, listener disbelieves, unintended actor overhears, report rejected as insufficient, or surface text fails validation.

## Speech act shape

A speech proposal should be able to represent:

```text
kind
speaker
listeners/audience/channel
intended social/institutional purpose
typed claims carried
claim modes: asserted_from_belief, hearsay, speculation, question, denial, lie, promise, threat, command
source belief or record references
asserted confidence/uncertainty
privacy/social context
role/procedure context
validation requirements
```

A committed speech event records the validated structured act. Surface text is derived or attached as non-authoritative rendering.

## Truthful assertion, accusation, speculation, lie, and hallucination

The same surface sentence can represent different acts.

```text
“Mara took the coins.”
```

Possible structured meanings:

- truthful assertion if speaker has adequate source claims;
- accusation/suspicion if speaker has partial actor-known basis;
- speculation if uncertainty is explicit;
- lie if speaker does not believe it or has reason to doubt it and intends deception;
- joke/deflection if social context supports it;
- invalid hallucination if no actor-bounded act supports it.

The actor must have authority to perform the selected act. A language model's plausible sentence is not a claim the world must accept.

## Listener interpretation

Listeners do not copy beliefs. They interpret speech through:

- trust in speaker;
- relationship;
- speaker role/status;
- listener prior beliefs;
- plausibility;
- known evidence and contradictions;
- social context;
- emotional state;
- bias;
- speech act kind;
- asserted confidence;
- institutional procedure if applicable.

A committed speech chain may look like:

```text
SpeechActCommitted
-> ListenerObservedSpeech
-> ListenerInterpretedClaim
-> BeliefUpdatedFromSpeech
-> PossibleRumorSeed
-> PossibleReactionGoal
```

## LLM allowed uses

LLMs may later be used for optional richness:

- render structured speech acts into varied prose;
- paraphrase records or notices;
- summarize actor-known information;
- summarize debug/no-human simulation results in debug contexts;
- parse player or NPC freeform text into candidate structured speech acts;
- propose repair text when a candidate act is invalid;
- style variation behind deterministic mocks.

These are surfaces. They are not authority.

## LLM forbidden uses

LLMs may not:

- decide truth;
- mutate authoritative state;
- create hidden facts;
- create clues or evidence;
- grant unearned knowledge;
- plan authoritative agents;
- choose actions;
- create quests;
- recognize completion;
- decide guilt, proof, sanction, payment, or institutional acceptance;
- silently correct stale information;
- bypass preconditions;
- summarize uncertainty as certainty in embodied mode;
- receive hidden truth in embodied prompt packets;
- make tests depend on live nondeterministic output.

## Prompt packet boundary

Prompt packets must be actor-filtered, purpose-scoped, minimal, and auditable.

A prompt packet for embodied rendering may include:

```text
speaker identity as known to the situation
listener identity as known
speech-act kind
source claims and source belief references available to speaker
asserted uncertainty
relationship/tone tags if actor-known or authored personality state
visible surroundings available to participants
record text/claims being paraphrased if accessible
```

It must not include:

```text
unobserved ground truth
true culprit
hidden item location
other actor private beliefs
debug notes
human possession history as fact
quest completion state
institutional facts unknown to speaker/listener
```

Do not include hidden truth and hope the LLM obeys.

## LLM rendering output

An LLM rendering output should align to structured acts.

Example:

```yaml
LanguageSurfaceOutput:
  surface_text: "I heard something by the room, but I don't know what it was."
  aligned_speech_acts:
    - kind: Tell
      claim: HeardNoiseNearTomasRoom
      asserted_confidence: low
      source_belief: belief_elena_noise_near_room
  unsupported_claims: []
  uncertainty_preserved: true
```

Only the structured, validated speech act can be committed. Surface text alone is never authoritative.

## Freeform parsing

Freeform player speech is not a v1 requirement. When added, it must follow:

```text
player text
-> parser proposes candidate speech acts and typed claims
-> actor-knowledge validation
-> lie/speculation/question/report classification
-> unsupported claim repair/rejection
-> optional player confirmation
-> ordinary speech action pipeline
-> committed speech event
```

A player controlling Tomas cannot truthfully assert “Mara stole my coins” unless Tomas has modeled support. The player may choose accusation, speculation, or lie if those acts are valid and carry risk.

## Claim extraction from generated utterances

If an LLM-generated utterance appears to share information, a deterministic or validated extraction layer must map it to candidate typed claims with source belief references before any belief transfer occurs.

Belief transfer occurs only through committed speech/perception events.

Unsupported concrete claims must be rejected, dropped, downgraded to speculation when valid, regenerated, or replaced by deterministic template.

A surface sentence cannot smuggle in:

- hidden facts;
- new evidence;
- identity certainty not held by speaker;
- stale-info correction;
- institutional recognition;
- quest status;
- payment promise not backed by actor/institution state.

## Lying versus invalid output

A lie is valid only when the speaker intentionally performs a lie/deception speech act with motive, risk, audience, and claim structure.

Invalid LLM output is not automatically diegetic lying. If the model invents “I saw Mara take it” but the actor has no such belief/memory and no validated lie intent, the output is rejected or repaired. The world does not absorb it.

## Records and paraphrase

LLMs may paraphrase records, but structured record claims remain authoritative.

A paraphrase must preserve uncertainty, source, issuer, staleness, proof limits, and scope. It may not upgrade “Tomas reports coins missing” into “Mara stole Tomas's coins.”

Embodied paraphrase must be based only on records accessible and interpretable by the current actor.

## Conversation logs

Conversation logs are event-linked projections.

Embodied logs:

- show what the current actor heard or said;
- preserve source and uncertainty;
- avoid hidden truth labels;
- distinguish actor notebook from human/debug notes.

Debug logs may show hidden speaker belief, lie status, validation reports, listener interpretation, belief deltas, unsupported LLM output, and prompt packet boundaries.

## Deterministic tests and LLM-disabled mode

Tracewake must run with LLMs disabled.

LLM-disabled mode uses:

- deterministic templates;
- structured speech menus;
- structured records;
- deterministic summaries;
- test fixtures;
- deterministic mocks for parse/render surfaces.

Disabling LLMs must not remove core gameplay, reports, rumors, lies, records, agent planning, TUI playability, no-human simulation, or replay.

## Future companion and party conversation

Future companion/party/adventure conversation is allowed only as ordinary social machinery.

Recruitment, refusal, loyalty, promises, pay, fear, injury, morale, betrayal, help, secrets, warnings, jokes, and arguments must be structured speech/social acts over typed claims, beliefs, values, needs, relationships, and obligations.

An LLM may make the exchange feel richer. It may not make the companion know hidden facts, accept party membership without cause, stay loyal because of player privilege, or recognize quest completion.

## Acceptance checks

Speech and LLM features are not accepted unless they test:

```text
structured speech event commit
actor-knowledge validation
typed claim extraction/source references
lie vs speculation vs assertion vs invalid hallucination
listener interpretation and belief provenance
overhearing if modeled
institutional report creation only through procedure
deterministic templates
LLM-disabled operation
invalid LLM output rejection/repair
no hidden truth in prompt packet
freeform input cannot grant knowledge
surface prose cannot mutate authoritative state
```

If language can invent reality, the feature is not Tracewake.

## 2026 hardening: language is a surface over structured speech acts

LLMs, templates, parsers, and prose renderers may help with language. They must not become simulation brains.

Allowed LLM or language-surface roles, behind validation:

- render a structured speech act into natural language;
- parse player/NPC utterance into candidate structured speech acts;
- summarize actor-known memories for prompt compression;
- suggest phrasing consistent with an actor's beliefs, personality, relationship, speech style, and context;
- classify ambiguous utterance candidates for deterministic validation;
- generate non-authoritative debug summaries clearly marked as debug.

Forbidden roles:

- mutate authoritative state directly;
- decide that a claim is true;
- decide action success;
- choose agent goals from hidden prompt context;
- create records/notices/bounties/institutional facts without validated procedure;
- reveal hidden truth through dialogue;
- merge human/player knowledge into actor knowledge;
- act as a drama director;
- make live nondeterministic output required for replay or acceptance tests.

## Prompt packet rule

Any future LLM prompt packet for actor speech must be actor-filtered. It may include only information available to the speaker or listener role as appropriate:

- current perceived context;
- actor beliefs, memories, expectations, intentions, needs, role, relationship, and mood where modeled;
- speech act purpose and allowed propositions;
- known records/notices/read artifacts;
- recent actor-known events;
- stylistic constraints;
- forbidden hidden facts and validation schema.

A prompt packet must not include hidden truth "for better writing" if the output could influence ordinary actor speech, listener beliefs, or action proposals.

## Lies, misunderstanding, persuasion, and hearsay

A speech act can create beliefs only through listener interpretation and modeled uptake.

- A lie is speaker-relative: it depends on what the speaker believes and intends.
- A misunderstanding is listener-relative: it depends on perception, language, trust, knowledge, context, and ambiguity.
- Persuasion modifies belief/confidence or intention only through modeled social reasoning.
- Hearsay and rumor carry source quality and may degrade.
- A true statement by a speaker who lacks basis may be a guess, rumor, or coincidence, not authoritative truth.

LLM wording can render these states. It cannot decide them.

## Determinism and replay

Acceptance tests must run without live LLM calls. Future LLM surfaces require deterministic mocks or stored validated outputs for replay. The event log commits structured speech acts and their validated consequences, not raw language as authority. Raw text may be stored as surface artifact, evidence for debugging, or record content where modeled, but the simulation meaning must be structured enough to validate and replay.
