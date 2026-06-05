# LLM Language Surface Boundary

## Core claim

LLMs may make language more natural. They must not become the authoritative simulation brain.

Correct model:

```text
structured world state
 -> actor-filtered prompt packet
 -> generated/proposed language
 -> structured speech-act proposal
 -> validation
 -> committed speech/action event
 -> observation/belief update
```

The LLM renders and proposes. The kernel validates and decides.

## Allowed uses

LLMs may later be used for:

- NPC dialogue surface;
- personality-specific phrasing;
- rumor phrasing;
- notice prose;
- case-file summary;
- memory summary;
- conversation recap;
- player-facing paraphrase;
- parsing player free text into proposed speech acts;
- optional reflection text from structured memories.

These uses remain optional surfaces.

## Forbidden uses

LLMs may not:

- decide ground truth;
- mutate authoritative state directly;
- invent hidden evidence;
- determine guilt;
- bypass action preconditions;
- create quests;
- choose agent plans in v1;
- grant NPCs unearned knowledge;
- silently correct stale information;
- create institutional records without action/procedure;
- decide success/failure of world actions;
- override deterministic tests;
- summarize uncertainty as certainty.

## Prompt packet

```yaml
SpeakerPacket:
  actor_id: actor_elena
  current_place: tavern_common_room
  visible_context:
    - actor_player_body_present
  needs:
    fatigue: 61
    hunger: 22
  current_intention: EatAndRest
  relevant_beliefs:
    - proposition: HeardNoiseNearTomasBedroom
      confidence: 0.34
      source: direct_sound_observation
  relationship_to_listener:
    trust: 0.52
  personality_speech:
    cautious: high
    directness: low
  forbidden:
    - do_not_assert_ground_truth
    - do_not_mention_unheld_beliefs
    - preserve_uncertainty
    - do_not_create_new_facts
```

Prompt packets must be actor-filtered. Do not include hidden truths and hope the model obeys.

## Output contract

```yaml
DialogueOutput:
  surface_text: "I don't know what I heard. Something moved near Tomas's room, maybe."
  speech_acts:
    - kind: Tell
      proposition: HeardNoiseNearTomasBedroom
      confidence_asserted: 0.34
      source_belief: belief_elena_noise
    - kind: Hedge
      proposition: NoiseMayHaveBeenNothing
      confidence_asserted: 0.55
```

Surface text is not authoritative. Speech acts are proposed, then validated.

## Validation

Validation checks:

- speaker holds the belief, is asking, speculating, or is intentionally lying;
- asserted confidence does not exceed allowed confidence unless lying/bluffing is modeled;
- speaker has motive/social willingness to reveal;
- listener can understand;
- speech act is socially permitted or knowingly violated;
- no unsupported concrete fact is introduced;
- uncertainty is preserved;
- structured claims match surface text closely enough;
- current actor knowledge restrictions are obeyed for player speech.

Invalid outputs are repaired, regenerated, downgraded to speculation, or rejected. Rejection may be logged for testing.

## Belief transfer

```text
SpeechActCommitted
 -> ListenerObservesSpeech
 -> ListenerInterpretsClaim
 -> BeliefUpdate
 -> PossibleRumorSeed
```

Listener confidence depends on trust, credibility, relationship, plausibility, evidence, status, context, and prior beliefs.

## Lies vs hallucinations

A valid lie is intentional false communication with motive and risk.

An invalid hallucination is an LLM inventing concrete facts not available to the speaker and not represented as deliberate lie, speculation, joke, or confabulation.

Hallucinations are rejected. Lies are committed only through modeled speech acts.

## Player free text

Player free text should be parsed into proposed speech acts.

Tomas cannot truthfully say:

```text
"Mara stole my coins."
```

unless he has actor-known support. He may:

```text
"My coins are missing."
"I suspect Mara."
"Someone was near my room."
"I think Mara had reason."
```

or he may lie deliberately if the action pipeline allows that social act.

## Determinism

Authoritative simulation must remain replayable.

If LLMs are used:

- store prompt packet;
- store model identifier;
- store output;
- store validation result;
- commit only typed validated actions/speech acts;
- use deterministic mocks in tests;
- support LLM-disabled operation.

## Minimal first use

Start with deterministic templates and mocks:

- notice paraphrase;
- speech-act-to-dialogue rendering;
- case summaries;
- memory summaries;
- conversation recap.

Open-ended NPC conversation comes after core epistemics, speech acts, TUI, and validation are solid.

## Anti-patterns

- LLM as planner;
- LLM creates clue;
- LLM decides culprit;
- LLM gives NPC hidden truth;
- LLM summarizes stale rumor as fact;
- prompt includes ground truth and relies on obedience;
- dialogue text directly mutates state;
- tests depend on live model output;
- “natural conversation” ships before structured speech acts work.
