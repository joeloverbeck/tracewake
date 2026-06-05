# Speech Acts, Language Surfaces, and LLM Boundary

## Status

This document defines Tracewake's speech-act and language-surface architecture.

V1 uses structured speech-act menus and deterministic templates. Freeform player speech and LLM parsing are deferred until validation, actor-knowledge filtering, and deterministic test harnesses are solid.

## Core rule

Language is a surface over structured simulation state. It is not simulation authority.

```text
actor beliefs/intentions/social context
 -> structured speech-act proposal
 -> action pipeline validation
 -> committed speech event
 -> observation by listeners
 -> interpretation/belief update
 -> optional deterministic rendering
```

An LLM may later render or parse proposals. The kernel validates and decides.

## V1 speech model

V1 supports structured speech-act menus/templates.

Required speech acts for first slice:

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

Additional canonical speech-act vocabulary may include deny, confess, threaten, bargain, recruit, warn, apologize, joke, and repair clarification, but first implementation should stay small.

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
    - proposition: ElenaHeardPossibleNoiseNearRoom
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

A committed speech event records the validated structured act. Surface text is derived.

## Speech as action

Speech acts pass through the action pipeline when they affect the world.

Checks include:

- speaker can speak/communicate;
- listener is present or reachable through a channel;
- speaker has belief, intention to lie, intention to speculate, or intention to ask;
- social permission or willingness to violate norms;
- role authority for commands/reports/testimony;
- privacy and overhearing conditions;
- time cost and interruption;
- listener perception and interpretation.

Speech can fail:

- listener absent;
- listener refuses attention;
- office closed;
- language mismatch;
- social fear blocks report;
- speaker chooses to withhold;
- listener disbelieves;
- overheard by unintended actor;
- report rejected as insufficient.

## Truthful assertion, lie, speculation, question

The same surface sentence may represent different structured acts.

```text
"Mara took the coins."
```

Possible structured meanings:

- truthful assertion if speaker has learned strong evidence;
- accusation/suspicion if speaker has partial basis;
- lie if speaker does not believe it and intends deception;
- speculation if marked uncertain;
- joke/deflection if social context supports it.

The actor must have authority to perform the selected act. A hallucinated fact from a language model is none of these.

## Listener interpretation

Listeners do not copy beliefs directly. They interpret speech using:

- trust in speaker;
- status/role credibility;
- relationship;
- prior beliefs;
- plausibility;
- evidence known to listener;
- social context;
- emotional state;
- bias;
- speech act kind;
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

Conversation logs are projections and/or event-linked records.

Embodied conversation log:

- shows what the current actor heard or said;
- preserves uncertainty and source;
- does not annotate hidden truth;
- distinguishes actor notebook from human/debug notes.

Debug conversation log:

- may show hidden speaker belief, lie status, validation report, listener interpretation, and belief deltas.

## Deterministic rendering

V1 uses deterministic templates for surface text.

```yaml
SurfaceTemplate:
  speech_act_kind: Report
  tone: cautious
  template: "I need to report missing property. {item} should have been in {place}."
  preserves_uncertainty: true
  supported_claim_modes: [asserted_from_belief, hearsay, speculation]
```

Rendering must not add unsupported concrete claims.

## Freeform player speech is deferred

Freeform player speech is not a v1 requirement. When added, it must be parsed into candidate speech acts, then validated.

Required future flow:

```text
player text
 -> parser proposes structured speech acts
 -> actor-knowledge validation
 -> lie/speculation/question/report classification
 -> unsupported claim repair/rejection
 -> player confirmation if needed
 -> speech action pipeline
 -> committed speech event
```

A player controlling Tomas cannot truthfully assert "Mara stole my coins" unless Tomas has learned support. The player may choose an accusation, speculation, or lie if those acts are available and validated.

## LLM allowed uses

LLMs may later be used for:

- rendering structured speech acts into prose;
- paraphrasing notices or records;
- summarizing actor-known information;
- summarizing debug/no-human simulation results;
- parsing proposed player utterances into candidate speech acts;
- style variation behind deterministic test mocks;
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
- decide action success/failure;
- decide guilt/proof/sanction;
- silently correct stale information;
- make tests depend on live nondeterministic output;
- summarize uncertainty as certainty;
- include ground truth in embodied prompt packets.

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

Do not include hidden truth and hope the LLM obeys.

## LLM output contract

```yaml
LanguageSurfaceOutput:
  surface_text: "I heard something by the room, but I don't know what it was."
  aligned_speech_acts:
    - kind: Tell
      proposition: HeardNoiseNearTomasRoom
      confidence_asserted: 0.34
      source_belief: belief_elena_noise_near_room
  unsupported_claims: []
  uncertainty_preserved: true
```

Only the structured, validated speech act can be committed. Surface text alone is never authoritative.

## Validation, rejection, repair

Validation checks:

- speaker holds belief, asks a question, speculates, or intentionally lies;
- asserted confidence is allowed;
- no unsupported concrete fact is introduced;
- uncertainty is preserved;
- listener can understand;
- social context permits or actor willingly violates;
- surface text aligns with structured act;
- prompt packet contained no forbidden truth;
- deterministic mock equivalent exists for tests.

Invalid output may be:

- rejected;
- repaired by dropping unsupported claims;
- downgraded to speculation;
- regenerated;
- replaced by deterministic template;
- logged as validation failure.

Repeated validation failure disables that surface for the run/test.

## LLM-disabled operation

Tracewake must run with LLMs disabled.

LLM-disabled mode uses:

- deterministic templates;
- structured menus;
- structured records;
- deterministic summaries;
- test fixtures and mocks.

No feature may require a live LLM to preserve simulation correctness.

## Acceptance implications

Speech/language features must test:

- structured speech event commit;
- actor-knowledge validation;
- lie vs hallucination distinction;
- listener belief update from trust/source;
- overhearing if modeled;
- report to institution creates record only through procedure;
- deterministic template output;
- LLM-disabled operation;
- invalid LLM output rejection;
- no hidden truth in prompt packet;
- freeform input, if enabled later, cannot grant knowledge.

## Anti-patterns

- LLM as NPC mind.
- Dialogue text directly mutates state.
- Player says any fact as truth because the player knows it.
- NPC reveals culprit because prompt included event log truth.
- Report creates case file without listener and record event.
- Natural conversation ships before structured speech acts work.
- Tests depend on a live model's wording.
- A lie is treated as invalid merely because it contradicts ground truth.
- A hallucination is accepted as a lie without actor motive and validation.
