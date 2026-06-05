# LLM Dialogue and Text Boundary

## Core claim

LLMs can greatly improve enjoyment by making agents speak naturally from their beliefs, needs, plans, and personalities. They must not become the authoritative simulation brain.

Correct model:

```text
structured world state
 -> prompt packet
 -> generated dialogue/prose
 -> structured speech-act proposal
 -> validation
 -> observation/belief update events
```

The LLM renders and proposes. The kernel validates and decides.

## Allowed uses

NPC dialogue surface, personality-specific speech style, rumor phrasing, notice prose, case-file summary, memory summary, conversation recap, player-facing paraphrase, optional reflection text from structured memories.

## Forbidden uses

Deciding ground truth, inventing hidden evidence, mutating world state through prose, determining guilt, bypassing preconditions, creating quests, planning authoritative actions without validation, giving NPCs knowledge they do not have, and silently correcting stale information.

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
```

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

Check whether speaker holds the belief or is intentionally lying; whether speaker has motive to reveal; whether listener can understand; whether the speech act is socially permitted or knowingly violated; whether generated text introduces unsupported facts; and whether uncertainty is preserved.

Invalid outputs are repaired, regenerated, downgraded to speculation, or rejected.

## Belief transfer

```text
SpeechActCommitted
 -> ListenerObservesSpeech
 -> ListenerInterpretsClaim
 -> BeliefUpdate
 -> PossibleRumorSeed
```

Listener confidence depends on trust, credibility, relationship, plausibility, and context.

## Lies vs hallucinations

A valid lie is intentionally false communication with motive and risk. An invalid hallucination is the LLM inventing concrete facts not available to the speaker and not represented as deliberate lie/confabulation.

## Player dialogue

Free player text should be parsed into proposed speech acts, validated against current actor knowledge, then committed or blocked. Tomas cannot truthfully say “Mara stole my coins” unless he has learned it. He may say “I suspect Mara,” “Someone took my coins,” or lie deliberately if willing.

## Determinism

Authoritative simulation must remain replayable. Store prompts, model ids, outputs, and validation results. Golden tests use deterministic mock LLM outputs. Core mechanics must work with LLM disabled.

## Minimal first use

Start with notice paraphrase, speech-act-to-dialogue rendering, case summaries, and memory summaries. Open-ended NPC conversation comes later.
