# Player Model, TUI, and UI

## Core claim

The player is a temporary controller, not the protagonist of reality. The UI lets the player perceive, decide, and intervene from the current actor’s position without granting ground-truth omniscience in normal play.

## Controller model

```yaml
HumanController:
  id: human_0
  attached_actor: actor_mara
  possession_history:
    - actor_tomas
    - actor_mara
  mode: embodied
```

Possession changes input binding. It does not reset suspicion, inventory, relationships, injuries, or obligations.

## Possession modes

Non-diegetic possession is default for prototypes. The world does not know possession exists. Diegetic possession is a future scenario/domain option with metaphysics, memory leakage, consent, detection, law, religion, and risk.

## Actor knowledge vs player knowledge

The human may know Mara stole the coins. Tomas may not.

Actor speech, testimony, accusation, institutional claims, and persuasion depend on actor knowledge. Player memory may guide exploration, but actor-formal action cannot assert unsupported facts.

## UI assistance

The TUI may list actions, explain why-not, sort beliefs, show inventory, display local map, and maintain notebooks. This is allowed if the final action is NPC-possible and actor knowledge restrictions apply.

## TUI as first real interface

The TUI should be playable, keyboard-driven, fast, readable, inspectable, and reusable in architecture.

Recommended panes:

```text
+----------------------+----------------------------+
| Local view / map     | Current actor              |
| rooms, exits, actors | needs, intention, status   |
+----------------------+----------------------------+
| Nearby affordances   | Beliefs / leads            |
| actions, objects     | actor-known information    |
+----------------------+----------------------------+
| Event/conversation log / prompts                  |
+---------------------------------------------------+
```

## Embodied mode

Shows current actor’s perceived place, nearby actors/objects, available actions, needs/status, actor-known beliefs/leads, conversations, and accessible notices/records.

Hides ground truth, omniscient event graph, hidden beliefs, true culprit, and exact locations known only to others.

## Simulation/debug mode

Mandatory during development. Shows event log, causal graph, belief inspector, institution records, trace map, stale information, planner decisions, LOD state, and replay.

## Lead notebook

The notebook distinguishes:

```text
Known by player
Known by current actor
Claimed by notice
Rumored
Recorded by authority
Observed directly
Inferred
Contradicted
Stale
```

## Objective markers

Markers represent beliefs, reports, maps, records, or observations. If a notice claims bandits are near the quarry and they moved yesterday, the marker remains wrong until updated.

## Why-not UI

Explain constraints causally:

```text
You cannot accuse Mara with evidence: Tomas does not know evidence linking her.
```

```text
You cannot claim payment now: the office is closed and the payment ledger is locked inside.
```

## Conversation UI

Show utterance text, speaker/listener, conversation history, and learned claims. Developer overlay may show structured speech acts. Embodied mode must not label lies unless the actor has reason.

## Time controls

Pause, step, slow/normal/fast, wait, sleep as actor action, travel time, accelerated simulation, and accessible missed-event summaries.

## Reuse for graphical UI

Keep domain presentation separate from terminal rendering:

```text
Simulation core -> UI view models -> TUI renderer -> future graphical renderer
```

## Minimal TUI

Local map/list of places, current actor status/intention, nearby actors/objects, action menu, possession switch, conversation/report interface, notice board, lead notebook, actor beliefs, debug event log, causal inspector, and why-not explanations.
