# Player Model and UI

## Core claim

The player is a temporary controller, not the protagonist of reality.

The UI’s job is not to tell the player what the story is. Its job is to expose artifacts, beliefs, leads, uncertainty, and causal chains without granting omniscience.

## Controller model

```yaml
HumanController:
  id: human_0
  attached_actor: actor_mara
  permissions:
    - issue_player_input
    - inspect_ui_available_to_attached_actor
  possession_history:
    - actor_tomas
    - actor_mara
```

Possession changes input binding. It does not reset suspicion, inventory, relationships, injuries, or obligations.

## Actor knowledge vs player knowledge

The human may know:

- Mara stole the coins;
- Tomas does not know;
- Elena heard a noise;
- the guard suspects Oren.

If currently controlling Tomas, the actor Tomas should not be able to make a direct accusation using knowledge only gained while controlling Mara unless the game explicitly supports supernatural cross-body knowledge.

Default rule:

```text
Actor action affordances depend on actor knowledge.
Player memory may guide exploration, but actor speech/evidence claims require modeled access.
```

## Possession design options

### Option A — Pure controller swap

The human keeps metaknowledge, but actors do not.

Pros:

- easiest to understand;
- supports forensic play;
- keeps simulation clean.

Cons:

- player can metagame physically.

### Option B — Diegetic possession

Possession is a supernatural ability known or discoverable in-world.

Pros:

- metaknowledge can become fiction;
- creates risk of possession detection.

Cons:

- adds major setting assumptions;
- may distract from foundational simulation.

### Recommendation

Start with Option A for development. Treat it as a debug/gameplay camera. Decide later whether to fictionalize it.

## UI layers

### 1. Embodied view

What the current actor can perceive directly.

- visible objects;
- heard sounds;
- nearby speech;
- accessible containers;
- readable notices;
- current bodily needs;
- immediate social affordances.

### 2. Actor mind view

What the current actor believes, remembers, intends, fears, and wants.

- beliefs with confidence;
- current intention;
- suspected contradictions;
- relationships;
- obligations;
- emotional state.

This should be optional but available. It is central to the game’s identity.

### 3. Public artifact view

Things accessible to the actor:

- notice board;
- ledgers they are allowed to see;
- rumors heard;
- contracts;
- maps;
- price lists;
- warrants.

### 4. Player notebook

A player-authored or UI-assisted record of leads.

The notebook can remember across possessions because it belongs to the human interface, but it should distinguish:

```text
Known by me as player
Known by current actor
Claimed by notice
Rumored by tavern
Recorded by authority
Observed directly
```

### 5. Forensic/debug view

During development, expose ground truth and causal graphs. Later, decide how much becomes a player-facing “investigation mode.”

## Leads, not quests

The UI should track leads.

```yaml
LeadCard:
  title: Missing coins at Tomas's house
  source: actor_tomas_report
  known_to_current_actor: true
  confidence: medium
  claims:
    - coins missing from strongbox
    - last seen yesterday evening
  unresolved_questions:
    - who had access?
    - was the lock forced?
    - did anyone hear movement?
```

A lead does not guarantee relevance, truth, or reward.

## Objective markers

Markers should represent beliefs or records, not truth.

Bad:

```text
Objective: Kill bandit leader. Marker points to exact cave.
```

Better:

```text
Notice says bandits may be near North Cave. Marker indicates reported location, last verified two days ago.
```

If the bandits moved, the marker remains wrong until updated.

## Causal inspector

A key UI/debug tool.

For any selected event:

```text
Event: Bounty notice posted
Caused by:
  - road attack report received
  - reeve assessed threat as high
  - funds available
  - clerk had paper and time
  - guard carried notice to board
Consequences:
  - three villagers read it
  - Oren formed intention to hunt bandits
  - bandits later heard rumor of bounty
```

This is the signature feature.

## Belief inspector

For selected actor:

```text
Tomas believes:
- coins were in strongbox last night, 86%, direct observation
- coins are missing, 97%, direct observation today
- someone entered bedroom at night, 45%, Elena testimony + missing coins
- Mara may be involved, 31%, prior distrust + rumor
```

Belief inspection makes wrongness legible.

## Why-not UI

When an action is unavailable, explain in terms of actor knowledge and world state.

Examples:

```text
You cannot accuse Mara with evidence: Tomas does not know any evidence linking Mara.
```

```text
You cannot claim the bounty here: the reeve's office is closed and the clerk has the payment ledger.
```

```text
You can break in, but current actor believes this is trespassing and fears being seen.
```

## Salience without authorship

The game may detect and surface interesting situations.

Allowed:

- “A public notice appears to be stale.”
- “Three people now suspect Oren.”
- “A theft case contradicts a witness statement.”
- “Your previous body is being questioned.”

Forbidden:

- spawning a theft because the player has not had content recently;
- making a witness appear because the story needs evidence;
- causing a confrontation because tension is low.

## Time controls

Because the world continues, time control matters.

Needed:

- pause;
- slow/normal/fast;
- sleep/wait as actor action;
- event notifications according to actor access;
- log filters;
- missed-event summaries derived from later-accessible information.

A player should be able to miss things. Missing things is part of the fantasy.

## UI anti-patterns

- Omniscient quest journal.
- Objective marker to truth.
- “Completed” stamps independent of institutional recognition.
- Universal crime indicator.
- Dialogue options that reveal facts the actor does not know.
- UI that hides uncertainty for convenience.
- Perfect map of all moving actors.

## Minimal UI for vertical slice

Build:

- embodied top-down or text view;
- actor needs and current intention;
- actor beliefs list;
- nearby objects/actions;
- notice board interface;
- lead notebook;
- event log/debug view;
- causal graph for selected event;
- possession switch menu;
- “known by player / known by actor” distinction.

This can be ugly. It must be truthful.
