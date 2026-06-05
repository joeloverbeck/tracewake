# Speech Acts, Language Surfaces, and LLM Boundary

## Status

This document defines structured speech acts, deterministic language surfaces, optional future LLM parsing/rendering, validation, and LLM-disabled operation.

V1 uses structured speech-act menus and deterministic templates. Freeform player speech and live LLM parsing are deferred until validation, actor-knowledge filtering, and deterministic test harnesses are solid.

## Core rule

Language is a surface over structured simulation state. It is not simulation authority.

```text
actor beliefs/intentions/social context
 -> structured speech-act proposal
 -> action pipeline validation
 -> committed speech event
 -> listener observation and interpretation
 -> belief/record/rumor/reaction updates
 -> optional deterministic rendering
```

An LLM may later render or parse proposals. The kernel validates and decides.

## Authority

This subsystem owns:

- speech-act vocabulary and structured proposal shapes;
- deterministic surface templates;
- prompt packet boundaries for optional LLM use;
- LLM output validation, repair, rejection, and fallback;
- deterministic mocks;
- LLM-disabled language operation.

It is denied:

- deciding truth;
- committing facts directly;
- choosing NPC actions;
- granting unearned knowledge;
- classifying guilt/proof/sanction;
- creating quests/clues;
- making tests depend on live nondeterministic output.

## V1 speech model

Required first-slice speech acts:

- greet;
- ask;
- answer;
- tell;
- report;
- gossip;
- refuse;
- lie;
- accuse/suspect;
- promise;
- command/instruct where role permits;
- testify/simple statement;
- withhold/deflect where useful.

Additional acts such as deny, confess, threaten, bargain, warn, apologize, joke, and repair clarification can be added only through the same contract.

## Speech act shape

```yaml
SpeechActProposal:
  kind: Report
  speaker: actor_tomas
  listeners: [actor_anna_clerk]
  intended_effect: request_authority_help
  propositions:
    - proposition: ItemMissing(coin_stack_01)
      mode: asserted_from_belief
      source_belief: belief_tomas_strongbox_empty
      asserted_confidence: 0.86
    - proposition: LastExpectedIn(coin_stack_01, strongbox_tomas)
      mode: asserted_from_belief
      source_belief: belief_tomas_coins_in_strongbox
      asserted_confidence: 0.82
    - proposition: NoiseHeardNearPlace(room_tomas_bedroom)
      mode: hearsay
      source_belief: belief_tomas_elena_noise_report
      asserted_confidence: 0.42
  social_context:
    place: reeves_office
    relationship_to_listener: ordinary_resident_to_clerk
    privacy: semi_public
  validation:
    requires_listener_attention: true
    requires_shared_language: true
    requires_actor_willingness: true
```

The committed speech event records the validated structured act. Surface text is derived.

## Speech as action

Speech acts pass through the action pipeline when they affect the world.

Checks include:

- speaker can communicate;
- listener is present/reachable and attentive enough;
- speaker has belief, question, speculation, lie intention, or withholding intention;
- role/social permission exists or actor is willing to violate;
- privacy and overhearing conditions;
- time cost and interruption;
- listener perception and interpretation.

Speech can fail or misfire:

- listener absent;
- listener refuses attention;
- office closed;
- language/channel mismatch;
- fear blocks report;
- speaker withholds;
- listener disbelieves;
- unintended actor overhears;
- report rejected as insufficient.

## Truthful assertion, lie, speculation, question

The same surface sentence can represent different structured acts.

```text
"Mara took the coins."
```

Possible structured meanings:

- truthful assertion if speaker believes strong evidence supports it;
- accusation/suspicion if speaker has partial basis;
- lie if speaker does not believe it and intends deception;
- speculation if marked uncertain;
- question if seeking confirmation;
- joke/deflection if social context supports it.

A hallucinated fact from a language model is none of these unless it is aligned to a validated structured act.

## Listener interpretation

Listeners do not copy beliefs directly. They interpret speech using:

- trust in speaker;
- status/role credibility;
- relationship;
- prior beliefs;
- plausibility;
- evidence known to listener;
- social context;
- emotion and bias;
- speech-act kind;
- asserted confidence;
- known contradictions.

```text
SpeechActCommitted
 -> ListenerObservedSpeech
 -> ListenerInterpretedClaim
 -> BeliefUpdatedFromSpeech
 -> PossibleRumorSeed
 -> PossibleReactionGoal
```

## Conversation logs

Embodied conversation logs show what the current actor heard, said, read, or inferred. They preserve uncertainty and source, and they do not annotate hidden truth.

Debug conversation logs may show hidden speaker belief, lie status, validation report, listener interpretation, and belief deltas.

## Deterministic rendering

V1 uses deterministic templates.

```yaml
SurfaceTemplate:
  speech_act_kind: Report
  tone: cautious
  template: "I need to report missing property. {item} should have been in {place}."
  preserves_uncertainty: true
  supported_claim_modes: [asserted_from_belief, hearsay, speculation]
```

Rendering must not add unsupported concrete claims.

## Freeform input boundary

Freeform player speech is future work. When added, it must parse into candidate speech acts, then validate.

```text
player text
 -> parser proposes structured speech acts
 -> actor-knowledge validation
 -> lie/speculation/question/report classification
 -> unsupported claim repair/rejection
 -> player confirmation where needed
 -> speech action pipeline
 -> committed speech event
```

A player controlling Tomas cannot truthfully assert “Mara stole my coins” unless Tomas has actor-known support. The player may choose accusation, speculation, question, or lie if valid.

## LLM allowed uses

LLMs may later be used for:

- rendering structured speech acts into prose;
- paraphrasing actor-known notices or records;
- summarizing actor-known information;
- summarizing debug/no-human runs with debug mode labels;
- parsing proposed player utterances into candidate speech acts;
- style variation behind deterministic mocks;
- repair suggestions for unsupported output.

These are optional surfaces.

## LLM forbidden uses

LLMs may not:

- decide truth;
- mutate authoritative state;
- plan agents;
- create hidden facts;
- grant unearned knowledge;
- create quests;
- bypass preconditions;
- decide success/failure;
- decide guilt/proof/sanction;
- silently correct stale information;
- make tests depend on live output;
- include hidden truth in embodied prompt packets.

## Prompt packet boundary

Prompt packets must be actor-filtered and minimal.

```yaml
PromptPacket:
  purpose: render_speech_act
  speaker: actor_elena
  listener: actor_tomas
  mode: embodied_surface
  allowed_context:
    current_place: tavern_common_room
    speaker_visible_context: [actor_tomas_present]
    source_beliefs:
      - belief_elena_noise_near_room
    relationship:
      trust: medium
    tone_tags: [cautious, tired]
  forbidden_context:
    - unobserved_theft_event
    - true_culprit
    - hidden_item_location
    - other_actor_private_beliefs
    - debug_notes
  output_contract:
    must_reference_only_source_beliefs: true
    must_preserve_uncertainty: true
    must_return_structured_alignment: true
```

Do not include hidden truth and hope the model behaves.

## Output validation

```yaml
LanguageSurfaceOutput:
  surface_text: "I heard something near the room, but I don't know what it was."
  aligned_speech_acts:
    - kind: Tell
      proposition: NoiseHeardNearPlace(room_tomas_bedroom)
      confidence_asserted: 0.34
      source_belief: belief_elena_noise_near_room
  unsupported_claims: []
  uncertainty_preserved: true
```

Only validated structured speech acts can be committed. Surface text alone is never authoritative.

Invalid output may be rejected, repaired by dropping unsupported claims, downgraded to speculation, regenerated, replaced by deterministic template, or logged as validation failure. Repeated validation failure disables that surface for the run/test.

## LLM-disabled operation

Tracewake must run with LLMs disabled.

LLM-disabled mode uses:

- structured menus;
- deterministic templates;
- structured records;
- deterministic summaries;
- test fixtures and mocks.

No feature may require a live LLM to preserve simulation correctness.

## Acceptance implications

Speech/language features must test:

- structured speech event commit;
- actor-knowledge validation;
- lie versus hallucination distinction;
- listener belief update from source/trust;
- overhearing where modeled;
- report-to-institution lifecycle;
- deterministic template output;
- LLM-disabled operation;
- invalid LLM output rejection;
- no hidden truth in prompt packet;
- freeform input, if enabled later, cannot grant knowledge.

## Anti-patterns

- LLM as NPC mind.
- Dialogue text directly mutates state.
- Player asserts any fact as truth because the human knows it.
- NPC reveals culprit because prompt included event-log truth.
- Report creates case file without listener and record event.
- Natural conversation ships before structured speech acts work.
- Tests depend on a live model's wording.
- A lie is rejected merely because it contradicts ground truth.
- A hallucination is accepted as a lie without actor motive and validation.
