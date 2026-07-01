# TUI Experience Overhaul — Research Report

**Target repository:** `joeloverbeck/tracewake`  
**Target commit:** `85dd8836508a58305eb84d70caf52bda088a9bde`  
**Report status:** analysis / recommendation report; not a numbered spec; not a `docs/4-specs/` ledger entry.

**Repository provenance note.** This report analyzes the user-supplied target commit only. It does not independently verify that the commit is the current `main`. All repository-state claims below are based only on files fetched from exact commit URLs whose paths appear in the uploaded manifest. The full fetch ledger is included in §10.3.

**Citation convention.** Repository citations use exact target-commit paths and line ranges observed in the fetched file bodies, e.g. `[REPO-A10: docs/1-architecture/... lines 37-86]`. External sources are cited with `[EXT-*]` labels and listed in §10.1. Repository file contents may mention other repositories; such mentions are treated as file content, not provenance failures.

---

## 1. Executive summary

Tracewake should stop presenting embodied play as a scrollback-heavy command transcript and become a full-screen, pane-oriented terminal interface that makes the possessed actor's situation legible at a glance. The target feel is not “a nicer REPL.” It is a subjective cockpit: a stable place-awareness panel, a body/routine panel, a co-present-actors panel, an action/affordance panel, a notebook/leads panel, and a bottom log for actor-known consequences, why-not feedback, and time-control summaries. A player should be able to glance at the right side of the screen and immediately see: “I am hungry and tired; I am at the bakery; Mara is here and appears to be working; Tomas is here but I cannot tell what he is doing; the door north is blocked; I can wait, eat, inspect, or continue my routine.”

The framework recommendation is **`ratatui` + `crossterm` in `tracewake-tui` only**, with `insta` or an equivalent snapshot dev-dependency for review artifacts. `ratatui` is immediate-mode and backend-driven; its `TestBackend` renders to an in-memory buffer for integration tests, which is exactly the right fit for Tracewake's existing pure `view-model -> rendered text` seam and golden/snapshot discipline [EXT-RATATUI-BACKENDS], [EXT-RATATUI-TESTBACKEND], [EXT-RATATUI-SNAPSHOTS]. `crossterm` supplies the cross-platform terminal/event/raw-mode substrate, including keyboard, mouse, resize, and raw-mode event handling [EXT-CROSSTERM], [EXT-CROSSTERM-EVENT]. Alternatives either sit too low (`termion`), own too much application structure (`cursive`, `tui-realm`), or leave the Rust/TUI crate boundary (`Textual`). Textual is valuable prior art for headless testing, but not the implementation stack [EXT-TEXTUAL-TESTING], [EXT-TEXTUAL-SNAPSHOT].

The co-present-awareness feature is **crate-crossing**. It cannot be solved by presentation alone. The current `EmbodiedViewModel` already carries `local_actors: Vec<VisibleActor>`, but `VisibleActor` contains only `actor_id`, and the renderer prints only the id. That is why “actor X is sleeping/eating/working” is not glanceable today. The correct enrichment is to extend the core view model with a closed, actor-known `ObservedActorActivityView` attached to visible/local actors. It must be produced only by a modeled perception/observation/projection boundary and sealed into the holder-known context. It must never be refreshed by the TUI from physical truth. If the possessed actor cannot perceive the activity, the activity is omitted or labelled as unknown; if it is stale or uncertain, the surface says so. This keeps the feature inside foundation 08's embodied TUI charter, foundation 14's actor-known truth firewall, architecture 03's holder-known context discipline, architecture 06's observation flow, and architecture 10's client/view-model boundary [REPO-F08: docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md lines 53-132], [REPO-F14: docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md], [REPO-A03: docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md], [REPO-A06: docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md], [REPO-A10: docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md lines 37-86].

The design's central hard requirement is **agent-drivability as a co-equal product requirement**, not a late testing concern. The fullscreen shell must be an adapter around deterministic state, not the authority boundary. The new architecture should introduce a `ScreenModel` / `ScreenDump` layer in `tracewake-tui` before the interactive shell: `EmbodiedViewModel -> EmbodiedScreenModel -> ratatui Buffer -> plain-text/structured screen dump`. Human terminals and automated agents both consume that seam. Golden tests and Claude-style scripted agents should be able to inject deterministic key/input intents, render at fixed terminal sizes, and inspect what is on screen as text/data. The existing `render_embodied_view(&EmbodiedViewModel) -> String` seam should remain as a canonical headless dump or be reimplemented as a wrapper over the new screen model; it must not be swallowed by an event loop.

No foundational amendment is recommended. The overhaul is a large implementation and architecture-spec pass, but the foundation already sanctions TUI-first playability, embodied actor-known surfaces, visible nearby actors, time controls, actor-safe why-not explanations, and permanent non-diegetic debug surfaces. The necessary changes are architectural/spec-level contract additions: richer visible-actor activity view-model fields, pane dispositions, headless buffer rendering, and acceptance upgrades. Foundation remains adequate.

---

## 2. Current-state assessment

### 2.1 What today's TUI actually is

Today's `tracewake-tui` is an executable line loop over stdin/stdout. `run_command_loop` prints `app.render_current_view()`, writes the prompt `tracewake>`, reads one line at a time from a `BufRead`, parses it with `parse_command`, dispatches a command, and writes another prompt [REPO-RUN: crates/tracewake-tui/src/run.rs lines 1-31]. The command vocabulary is textual: `help`, `view`, `notebook`, `wait` / `w`, `continue` / `c`, `bind`, `do`, numeric menu selections, and `debug ...` subcommands [REPO-INPUT: crates/tracewake-tui/src/input.rs lines 4-103].

This is excellent for deterministic testing and poor as an embodied UI. It is a transcript shell: the only “layout” is line order, the only focus model is what the user remembers from the last render, and the only navigation affordance is typing a command or action number. There is no persistent local-world dashboard, no stable place for needs/routine information, no pane that keeps visible actors in peripheral awareness, and no way to see the consequences of time passage without rereading fresh output.

The current pure render seam is good and must be preserved. `render_embodied_view(&EmbodiedViewModel) -> String` is a deliberately testable pure function [REPO-RENDER: crates/tracewake-tui/src/render.rs lines 12-20]. It already has debug-token hygiene, including a test that keeps `DEBUG NON-DIEGETIC` markers out of embodied output [REPO-RENDER: crates/tracewake-tui/src/render.rs lines 5-10]. `TuiApp::render_current_view` simply calls this function on `self.current_view()` [REPO-APP: crates/tracewake-tui/src/app.rs lines 201-203]. This seam should become the foundation of the new headless renderer, not a casualty of fullscreen interactivity.

### 2.2 Why it feels clunky and opaque

The clunkiness is structural:

- **No persistent spatial composition.** Place, exits, doors, containers, items, actors, needs, interval summaries, and actions are emitted as a single vertical list. The user's eye cannot rely on stable panel positions.
- **No focus or inspection model.** Text commands and numeric action entries do not express “focus the actor pane,” “inspect this actor,” “show why this action is disabled,” or “pin recent actor-known notices.”
- **No keybinding discoverability.** `help` exists, but the interface does not continuously show the available keys/actions in context. HCI guidance strongly favors visibility of system status and recognition over recall; the user should not have to remember which command reveals the next legal move [EXT-NNG-STATUS], [EXT-NNG-RECOGNITION].
- **No hierarchy between urgent and background information.** Fatigue, hunger, active intention, rejection reason, local actors, and action choices all have similar visual weight. In a simulation about ordinary life and subjective cognition, that is the wrong hierarchy.
- **Debug/operator output is present but not dashboarded.** Debug panels exist, but they are selected by command and emitted as line text rather than permanent, visibly non-diegetic tabs or panes.

### 2.3 Why co-present actor activity is not glanceable today

The view-model seam has the right home for the feature but not the data. `EmbodiedViewModel` includes `local_actors: Vec<VisibleActor>` alongside visible exits, doors, containers, items, carried items, semantic actions, embodied status, why-not, interval summary, notebook, and debug availability [REPO-VM: crates/tracewake-core/src/view_models.rs lines 18-42]. But `VisibleActor` is only:

```rust
pub struct VisibleActor {
    pub actor_id: ActorId,
}
```

[REPO-VM: crates/tracewake-core/src/view_models.rs lines 389-392].

The renderer therefore prints only:

```rust
lines.push("Actors:".to_string());
for actor in &view.local_actors {
    lines.push(format!("- {}", actor.actor_id.as_str()));
}
```

[REPO-RENDER: crates/tracewake-tui/src/render.rs lines 127-130].

The core perception path likewise records local actor facts as actor-known local actor identity facts, not activity facts. `current_place_knowledge_context` builds `local_actors`, and when it sees an `ActorKnownProjectionRecord::LocalActor`, it pushes `ActorKnownLocalActorFact::new(observed_actor_id, source_key)` [REPO-PERCEPTION: crates/tracewake-core/src/agent/perception.rs lines 265-283, 517-529]. That is truth-firewall compatible, but too thin for embodied play. It tells the TUI “this actor is locally visible/known,” not “this actor is visibly sleeping” or “this actor was last seen eating, maybe stale.”

### 2.4 Dependency state

The TUI crate currently depends only on `tracewake-core` and `tracewake-content` in normal dependencies; there is no `ratatui` or `crossterm` dependency [REPO-TUI-CARGO: crates/tracewake-tui/Cargo.toml lines 7-12]. `tracewake-core` has no normal dependencies at all [REPO-CORE-CARGO: crates/tracewake-core/Cargo.toml lines 7-10]. This confirms the migration boundary: UI framework dependencies belong in `tracewake-tui`, while core remains plain Rust/domain types.

### 2.5 Current acceptance assets to preserve

The repository has already ratified two constraints that this overhaul must consume rather than redesign:

1. **Two-hop playable-capability parity.** Archived spec 0046 scopes a contract where every playable capability needs simulation-capability-to-view-model evidence and view-model-to-rendered-surface evidence; it explicitly made TUI modernization out of scope, so this report fills a known gap rather than reopening parity [REPO-0046: archive/specs/0046_TUI_SIMULATION_PLAYABLE_CAPABILITY_PARITY_CONTRACT_AND_TWO_HOP_DRIFT_GUARD_HARDENING_SPEC.md lines 31-67].
2. **Kernel-owned time advancement and actor-known interval summaries.** Archived spec 0047 scopes a kernel-owned one-world-tick coordinator, repeated one-tick acceleration, and actor-known interval summaries that are structurally separate from debug reports [REPO-0047: archive/specs/0047_TUI_AUTHORITATIVE_WORLD_ADVANCE_DURATION_COMPLETION_AND_ACTOR_KNOWN_INTERVAL_SUMMARIES_SPEC.md lines 55-104].

The new fullscreen TUI must sit on top of those contracts. It must not move world rules into the TUI, create direct event application paths, or replace holder-known interval summaries with raw event diffs.

---

## 3. Target design

### 3.1 Design principles

The target interface should be governed by five rules.

**Embodied first.** The default screen is the possessed actor's situation. Debug/operator mode exists, remains permanent, and becomes much better, but it is never the primary play surface. Foundation 08 says the TUI is the first real client rather than a disposable debug console, and embodied mode must show actor-filtered reality while hiding hidden truth [REPO-F08: docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md lines 5-10, 53-75].

**Subjective, not omniscient.** Every embodied pane renders a typed view model produced from actor-known context. The TUI does not ask the physical state for “updated labels.” Architecture 10 explicitly forbids the TUI from holding a live physical-state handle or re-reading truth to freshen labels, routes, attributes, food, or blockers [REPO-A10: docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md lines 79-86]. The co-present-activity pane must obey the same rule.

**Recognition over recall.** The UI should continually show available actions, pane focus, hotkeys, and actor-safe why-not context. This is not mere polish: HCI heuristics make visibility of system status and recognition-over-recall core usability properties [EXT-NNG-STATUS], [EXT-NNG-RECOGNITION].

**Immediate-mode render, deterministic state.** The interactive shell should redraw from a screen model derived from the current `EmbodiedViewModel`; it should not keep its own simulation copy. Ratatui's model, where the application owns the event loop and redraws the UI, fits this discipline [EXT-RATATUI-FAQ].

**Headless is first-class.** Every fullscreen concept must have a headless text/data representation. If an agent cannot inspect the screen, the feature is not done.

### 3.2 Embodied layout

Recommended default terminal layout for 100x30-ish terminals:

```text
┌ Tracewake — Embodied: <actor-safe actor label> ──────────────────────────────┐
│ Place / Situation                         │ Self / Body / Routine            │
│ - perceived place label                    │ - hunger, fatigue, safety bands  │
│ - exits, doors, blockers                   │ - intention / routine            │
│ - nearby items, containers, traces         │ - carried items summary          │
│                                           │                                  │
│ Co-present actors                          │ Notebook / Leads                 │
│ 1. Mara — working (seen now)               │ - source-bound beliefs           │
│ 2. Tomas — activity not apparent           │ - possible leads / contradictions│
│ 3. someone nearby? — heard, uncertain      │                                  │
│                                           │                                  │
│ Actions / Affordances                      │ Details / Why-not                │
│ [1] eat bread                              │ selected action, actor-safe      │
│ [2] inspect Mara                           │ blocker facts, provenance labels │
│ [w] wait one tick  [c] continue            │                                  │
├ Recent actor-known changes / input hints ────────────────────────────────────┤
│ You noticed Mara still at the oven. No new actor-known interval notices.     │
└ ? help  Tab focus  Enter act  n notebook  w wait  c continue  q quit ────────┘
```

The exact box drawing is optional; the structure is not. The screen must preserve stable regions so the user can build peripheral memory. On narrow terminals, collapse into a single-column stack with headings and focus controls; do not omit safety-critical embodied facts silently. Color may help, but color must never be the only carrier of meaning.

#### Pane-to-view-model contract

| Pane | Renders | Contract boundary |
|---|---|---|
| Header / mode bar | Actor-safe identity, mode, focused pane, debug availability marker only if appropriate | Existing `viewer_actor_id`, `ViewMode`, `debug_available`; do not show hidden debug truth in embodied mode. |
| Place / Situation | perceived place label; visible exits, doors, containers, items, blockers | Existing `place_label`, `visible_exits`, `visible_doors`, `visible_containers`, `visible_items`; presentation-only grouping. |
| Self / Body / Routine | need bands, intention, routine, salient interruption, carried items | Existing `phase3a_status`, `carried_items`; presentation-only unless future status fields are needed. |
| Co-present actors | local visible actors plus observed public activity, staleness, uncertainty | Requires core enrichment of `VisibleActor`; crate-crossing, architecture-owned. |
| Actions / Affordances | semantic action entries, availability, menu/hotkey mapping | Existing `semantic_actions`; menu position is presentation-only under architecture 10. |
| Details / Why-not | selected action detail, actor-safe rejection, actor-visible facts, reason codes | Existing `last_rejection_why_not` / `WhyNotView`; no debug validator internals. |
| Notebook / Leads | beliefs, observations, contradictions, typed leads, possible leads | Existing `NotebookView`; presentation-only unless conversation/lead filters need new closed fields. |
| Recent actor-known changes | actor-known interval summary after wait/continue | Existing `TypedActorKnownIntervalSummary` from 0047; never raw event diff. |
| Input hints | pane-local keybindings and command hints | TUI-only; maps to typed UI intents, then semantic actions or core runtime commands. |

This layout consumes architecture 10's minimum embodied view-model schema and its two-hop/captured-projection rules. New fields are not rendered by default arms; each receives an explicit pane disposition [REPO-A10: docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md lines 37-86].

### 3.3 Co-present awareness surface

This is the headline product improvement. The panel should answer three actor-known questions:

1. **Who or what does the possessed actor perceive nearby?**
2. **What publicly observable activity does the possessed actor have reason to attribute to them?**
3. **How reliable/fresh is that attribution?**

Recommended presentation examples:

```text
Co-present actors
1. Mara — working at the oven          seen now · visual
2. Tomas — sleeping                    seen earlier · may be stale
3. Lio — activity not apparent         visible, no observable activity
4. someone nearby — moving?            heard · identity uncertain
```

The panel must not become an omniscient roster. The row taxonomy should be:

- **Direct observed activity:** The possessed actor directly perceives the actor and the activity at the current modeled observation boundary: “Mara is eating,” “Tomas is sleeping,” “Rin is working at the press.”
- **Indirect/uncertain activity:** The possessed actor has a modeled sensory clue without full identity or action certainty: “someone is moving nearby,” “Mara may be talking in the next room.” Use this only when actor-known doctrine allows the identity/activity claim.
- **Stale remembered activity:** The possessed actor previously observed an activity and has no fresher observation. Display with explicit staleness: “last seen sleeping earlier — may have changed.”
- **Visible but activity unknown:** Actor is present/visible, but the activity is not observable or not modeled as public: “activity not apparent.”
- **Hidden/occluded/unperceived:** no row. Omission is correct.

Prior art from roguelike and simulation interfaces supports the principle, not the metaphysics: line-of-sight and field-of-view systems deliberately gate what entities the player can perceive [EXT-ROGUEBASIN-FOV], [EXT-COGMIND-FOV], [EXT-HEXWORKS-FOV]. Dwarf Fortress demonstrates that simulation UIs benefit from surfacing creature jobs/statuses, but Tracewake must only do so through the actor-known firewall [EXT-DF-VIEW-JOB].

#### Source rule

A co-present activity row is legal only when core has produced an actor-known observation/projection item. Legal sources include perception, memory, inference, testimony, and records as defined by foundation 04; hidden physical truth is not a legal source [REPO-F04: docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md]. The observation/projection item must carry provenance and freshness sufficient for actor-relevant staleness labels, consistent with holder-known context doctrine [REPO-A03: docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md].

#### UI behavior

- Focusing a row opens an actor detail pane with only actor-known facts: observed activity, last observation source, known relationship/role if already actor-known, possible speech/action affordances, and actor-safe uncertainty.
- If the actor is visible but no activity is known, do **not** infer from ground-truth routine/intention. Say “activity not apparent.”
- If the actor is sleeping behind a closed door, do not show the actor or activity unless there is a modeled actor-known cue that justifies it. A sound may be represented as a sound, not as omniscient identity.
- If an activity is stale, stale wording is mandatory. The label should be actor-relevant, e.g. “seen earlier,” “remembered,” “reported,” “may be stale,” not raw internal event jargon.
- Activity labels should be typed/closed enough that the renderer must consciously handle each new category. Free prose generated below the TUI is allowed only as actor-safe summary text attached to a typed source; it may not create facts.

### 3.4 Self, needs, intention, and routine

The current renderer already emits need bands, intention summary, routine summary, and salient interruption from `phase3a_status` [REPO-RENDER: crates/tracewake-tui/src/render.rs lines 38-60]. The new UI should elevate this into a persistent self panel:

```text
Self
Hunger: severe — last changed by waiting
Fatigue: tired — sleeping would help
Safety: stable
Intention: get food
Routine: breakfast / continue available
Interruption: hunger severe
```

This is mostly TUI-only: it renders existing `Phase3AEmbodiedStatus`. If future specs need richer self-state, they should add closed fields to `Phase3AEmbodiedStatus` rather than ask the TUI to inspect core state.

### 3.5 Place, exits, objects, traces, and affordances

The place panel should render the existing actor-filtered place data with better hierarchy. Exits and blockers belong together; doors should be folded into exit rows when helpful; containers/items should be grouped by relevance. This is presentation-only as long as it uses `visible_exits`, `visible_doors`, `visible_containers`, `visible_items`, and `carried_items`.

Recommended presentation:

```text
Place: Bakery kitchen
Exits
- north: shop floor — door closed
- east: pantry — route known
Objects
- bread loaf (portable)
- flour bin (closed container)
Inventory
- kitchen knife (carried)
```

The TUI must not invent routes, object contents, or blockers. Architecture 10's captured-projection rule explicitly forbids re-reading truth to freshen blockers or labels [REPO-A10: docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md lines 79-86].

### 3.6 Actions, why-not, and leads

Actions should be visible continuously. The current `semantic_actions` list already provides semantic action ids, action ids, target ids, labels, availability, and provenance. The fullscreen UI should show a compact action list with stable semantic IDs available in the detail pane and transcript, while menu positions remain presentation-only as architecture 10 requires [REPO-A10: docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md lines 61-70].

Disabled actions should remain visible when they are actor-known affordances, but their why-not text must be actor-safe. Architecture 10 already requires why-not explanations to distinguish actor-facing explanations from debug truth and forbids leaking hidden targets, culprit labels, validator internals, or unexplained projection facts [REPO-A10: docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md lines 145-156]. The design should use this split directly:

```text
[3] open pantry door — not now
Why not: You believe the door is closed and locked.
Facts you can rely on: door seen locked earlier; key not in inventory.
```

Leads and notebook items should become glanceable but not quest markers. Foundation 08 allows actor-known contradictions, source-bound beliefs, rumors, records, memories, and possible leads while forbidding omniscient quest state [REPO-F08: docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md lines 53-75]. The notebook panel should therefore show source-bound, fallible leads with “how this may be wrong” information already present in `NotebookLeadEntry` [REPO-VM: crates/tracewake-core/src/view_models.rs lines 337-350].

### 3.7 Navigation, focus, and keybindings

Recommended focus model:

- `Tab` / `Shift+Tab`: move between panes.
- Arrow keys: move within focused list.
- `Enter`: activate selected action/detail if it maps to a legal typed intent.
- `?`: context help for focused pane.
- `w`: wait one authoritative tick when available.
- `c`: continue/advance-until using the settled kernel-owned continuation contract.
- `n`: focus notebook.
- `a`: focus actions.
- `p`: focus co-present actors (“people/presence”).
- `q`: quit / request confirmation if needed.

The keybinding layer is TUI-only and must translate to typed `UiIntent` values. `UiIntent` values then either select an existing semantic action or call an already-authorized runtime/controller method such as the settled time-control path. No key directly mutates world state.

Menus must carry two identities: a presentation index/hotkey for the screen, and a semantic action id for transcripts and submissions. This preserves the architecture rule that menu positions are presentation-only while semantic IDs remain stable [REPO-A10: docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md lines 61-70].

### 3.8 Time controls

The time-control UI should make the settled 0047 contract pleasant without weakening it.

Recommended controls:

```text
Time
[w] wait one tick       [c] continue routine / advance until salient stop
[p] pause after current tick (during continuation)
```

Rules:

- `wait` is one authoritative world tick, not a local possessed-actor clock.
- `continue` is repeated authoritative one-tick progression with deterministic stop conditions.
- The bottom log renders the actor-known interval summary, not raw event diffs or hidden due queues.
- Debug exact tick reports remain in debug/operator mode.

Foundation 08 already says waiting runs the simulation, acceleration/continuation is staged as repeated authoritative progression, and sleep summaries must be actor-known with no omniscient leakage [REPO-F08: docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md lines 257-303]. Architecture 10 likewise routes world-advancing controls through core/runtime and requires actor-facing interval displays to derive from holder-known/modelled observations/records/public cues [REPO-A10: docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md lines 98-133].

### 3.9 Conversation UI

Conversation should be a structured speech-act panel, not a freeform parser that mutates state. Foundation 11 governs the language boundary: LLM/prose surfaces may render or parse candidate speech but may not create simulation facts or bypass validators [REPO-F11: docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md]. The TUI target is:

```text
Conversation with Mara
Intent: ask / tell / accuse / offer / refuse
Topic: bread missing / door locked / work shift
Utterance preview: "Did you see who took the bread?"
Evidence attached: observation obs_... (actor-known)
[Enter] submit speech act  [Esc] cancel
```

If existing semantic actions already expose conversation affordances, the TUI renders them. If richer conversation choices are needed, that is crate-crossing and should add closed `ConversationAffordanceView` / speech-act view-model data owned by architecture 07 and architecture 10, not ad hoc parser behavior in the TUI.

### 3.10 Debug/operator redesign

Debug/operator mode should become a separate, full-screen dashboard with unmistakable non-diegetic framing. Foundation 08 and architecture 10 both require permanent debug surfaces, but they must be visibly non-diegetic and quarantined from embodied play [REPO-F08: docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md lines 82-92], [REPO-A10: docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md lines 160-176].

Recommended debug tabs:

- **Event log / replay frontier** — event stream, replay checksum, temporal frontier.
- **Projection rebuild** — projection status and rebuild diagnostics.
- **Holder-known context inspector** — compare actor-known context with provenance; never fed back into embodied view.
- **Beliefs / observations / contradictions** — debug views already represented by `DebugBeliefsView`, `DebugObservationsView`, `DebugEpistemicsView`.
- **Action validation / why-not** — validator reports split into actor-facing vs debug-only.
- **Scheduler / no-human** — due work, routine progress, no-human run diagnostics.
- **TUI seam diagnostics** — current screen model, pane dispositions, render snapshot metadata.

Every debug screen should carry the marker `DEBUG NON-DIEGETIC` or an equivalent locked marker and should be impossible to confuse with embodied mode. Debug commands require debug authority; they may show truth for inspection but must not write actor-known beliefs or alter embodied rendering.

---

## 4. Core enrichment required

### 4.1 What can be done in the TUI crate only

The following are presentation-only and should not cross into core:

- Pane layout, focus model, keyboard mapping, help overlays, scroll offsets, and responsive terminal-size behavior.
- Grouping and formatting existing place/exits/doors/items/containers/inventory fields.
- Elevating `phase3a_status` into a persistent self panel.
- Rendering existing `semantic_actions` as action lists with hotkeys and detail panes.
- Rendering existing `NotebookView` and `TypedActorKnownIntervalSummary` in better panes.
- Producing `ScreenModel`, `ScreenDump`, and ratatui buffers from existing view models.
- Modernizing debug panels as fullscreen tabs while retaining debug authority checks.

All of these live in `tracewake-tui`. They may add `ratatui`, `crossterm`, and test-only snapshot dependencies to `tracewake-tui`. They must not add UI dependencies to `tracewake-core` or `tracewake-content`.

### 4.2 What requires core/view-model enrichment

The co-present-activity feature requires new actor-known data in core.

#### Recommended view-model shape

Extend `VisibleActor` in `crates/tracewake-core/src/view_models.rs` from identity-only to a typed actor-known presence/activity view:

```rust
pub struct VisibleActor {
    pub actor_id: ActorId,
    pub display_label: String,
    pub presence: VisibleActorPresenceView,
    pub observed_activity: Option<ObservedActorActivityView>,
}

pub struct VisibleActorPresenceView {
    pub source_summary: String,
    pub observed_tick: Option<u64>,
    pub staleness_label: String,
    pub uncertainty_label: Option<String>,
}

pub struct ObservedActorActivityView {
    pub kind: ObservedActorActivityKind,
    pub actor_safe_summary: String,
    pub source_kind: ActorKnownActivitySourceKind,
    pub source_summary: String,
    pub observed_tick: Option<u64>,
    pub staleness_label: String,
    pub uncertainty_label: Option<String>,
}

pub enum ObservedActorActivityKind {
    Sleeping,
    Eating,
    Working,
    Moving,
    Speaking,
    Waiting,
    ContinuingRoutine,
    ApparentIdle,
    ActivityNotApparent,
}

pub enum ActorKnownActivitySourceKind {
    DirectPerception,
    IndirectPerception,
    Memory,
    Testimony,
    Record,
    Inference,
}
```

The exact names are spec-authoring choices; the important properties are closed enums, actor-safe summaries, source/freshness labels, and no UI-framework types. `ActivityNotApparent` can be either an enum variant or represented by `observed_activity: None` plus a presence label; choose the shape that best supports exhaustive renderer guards.

#### Projection source

The enrichment should flow through the existing actor-known context/projection seam:

1. Model public/observable activity facts in core as a typed observation/projection record, not TUI prose.
2. During perception/context construction, include observed activity only when the possessed actor's modeled perception can capture it.
3. Seal the activity into holder-known context/provenance, with source and frontier information.
4. `build_embodied_view_model` transfers the actor-known activity into `VisibleActor`.
5. `tracewake-tui` renders the field, with explicit disposition under the two-hop contract.

The current perception path already translates `ActorKnownProjectionRecord::LocalActor` into local actor facts [REPO-PERCEPTION: crates/tracewake-core/src/agent/perception.rs lines 517-529]. The new work should extend that family with a local-actor observed-activity fact rather than add a TUI-side lookup.

#### What not to do

- Do not derive activity in the TUI from `actor_id`, action registry, routine state, scheduler state, or physical actor state.
- Do not add ratatui/crossterm types to core view models.
- Do not collapse internal intentions into visible activity. “Wants to steal bread” is not an observable activity unless a modeled speech/record/observation source makes an actor-known claim.
- Do not show an actor's exact hidden routine, target, job assignment, or causal culprit label.
- Do not make stale activity look current.
- Do not introduce a wildcard renderer path such as “unknown future activity -> debug string.” Closed types need explicit presentation disposition.

### 4.3 Doctrine homes for the enrichment

This is a crate-crossing change and should be routed explicitly:

| Needed contract | Owning doctrine home | Substance to own |
|---|---|---|
| `VisibleActor` carries actor-known observed public activity, source, freshness, and uncertainty | `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` | Embodied view-model contract expands from visible actor identity to visible actor awareness. Two-hop presentation disposition required for every new field and enum variant. |
| Activity facts are captured at modeled perception/observation boundaries and sealed into holder-known context | `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` | Co-present activity is holder-known context material, not live truth. Provenance/freshness is mandatory. |
| Observing another actor's public activity becomes an observation/belief/memory candidate under existing information flow | `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` | Public activity observations are typed observation records with source discipline; they may later become beliefs/memories/leads, subject to contradiction/staleness. |
| Execution tests prove no hidden activity leakage and no stale/fresh mislabeling | `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` and `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` | Positive and negative fixtures must prove visible direct activity, occluded omission, stale labels, possession parity, and debug quarantine. |
| Snapshot/golden artifacts treat the actor panel as a first-class acceptance surface | `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` | Screen dumps and ratatui buffers join existing transcript/golden artifacts; no decorative locks. |

### 4.4 Content/data implications

`tracewake-content` should not own UI framework behavior, but it may need typed fixture support for observable public activities. For example, fixtures should exercise:

- co-present actor sleeping visibly in the same room;
- actor eating visibly;
- actor working visibly;
- actor present but activity not apparent;
- actor hidden/occluded and therefore absent;
- stale remembered activity after the actor has moved or changed state without fresh observation;
- indirect sound/record/testimony cases if the current epistemic model supports them.

Any content labels must be validated as source-bound domain data. They must not become a prose channel for facts that the simulation did not produce.

---

## 5. Framework recommendation

### 5.1 Recommended stack

Use:

- `ratatui` as the rendering/widget/layout layer in `tracewake-tui`.
- `crossterm` as the terminal backend/event/raw-mode layer in `tracewake-tui`.
- `insta` / `cargo-insta` or equivalent as a dev-dependency for buffer/text snapshots.
- A small internal `tracewake-tui` screen-model module that keeps ratatui widgets away from core view-model construction.

Why this wins:

- `ratatui` abstracts drawing through backends and supports layout/widgets without owning simulation authority [EXT-RATATUI-BACKENDS].
- `ratatui::backend::TestBackend` renders to an in-memory buffer intended for whole-TUI integration tests [EXT-RATATUI-TESTBACKEND].
- Ratatui's own testing guidance points to snapshot tests with `insta` / `cargo-insta`, matching Tracewake's existing golden/snapshot culture [EXT-RATATUI-SNAPSHOTS].
- `crossterm` is pure Rust and cross-platform, and its event module covers keyboard, mouse, resize, polling, raw mode, and asynchronous event-stream options [EXT-CROSSTERM], [EXT-CROSSTERM-EVENT].
- Ratatui keeps the application in charge of event loop, state, and redraw. That is a feature for Tracewake: the shell remains an adapter around typed view models and runtime commands [EXT-RATATUI-FAQ].

### 5.2 Alternatives considered and rejected

| Alternative | Strength | Why it loses for Tracewake now |
|---|---|---|
| `termion` | Low-level terminal control; pure Rust. | Too low-level and less directly aligned with cross-platform event/raw-mode expectations. It does not provide the widget/layout/test-buffer layer Tracewake needs. |
| `cursive` | Higher-level TUI framework with event loop, callbacks, focus, and backend support. | It owns more of the application structure. Tracewake needs an explicit pure screen-model/render seam and deterministic headless buffer path; a higher-level framework risks hiding those seams behind callbacks [EXT-CURSIVE], [EXT-RATATUI-FAQ]. |
| `tui-realm` | Ratatui-based framework with Elm/React-like structure and focus management. | Potentially useful later, but too much framework before Tracewake has its own screen model and parity guards. It would add another conceptual layer over the already-sensitive core/view-model/TUI boundary [EXT-TUIREALM], [EXT-RATATUI-FAQ]. |
| `Textual` | Excellent testing model with `run_test`, pilot key presses, and snapshot tests. | Python ecosystem, not Rust crate-boundary compliant. Use it as prior art for automated TUI driving, not as the implementation stack [EXT-TEXTUAL-TESTING], [EXT-TEXTUAL-SNAPSHOT]. |
| Keep line REPL and add prettier text | Minimal dependency change; preserves tests. | Fails the product goal. It cannot provide persistent panes, focus, at-a-glance co-present activity, or a modern embodied experience. |

### 5.3 Dependency-direction compliance

The dependency rule is simple:

```text
tracewake-core      -> no UI deps, no ratatui/crossterm, plain view-model types only
tracewake-content   -> no UI deps, fixture/domain data only
tracewake-tui       -> ratatui + crossterm + snapshot dev-deps allowed
```

This respects the current crate topology: `tracewake-core` currently has no normal dependencies, while `tracewake-tui` already depends on core/content [REPO-CORE-CARGO: crates/tracewake-core/Cargo.toml lines 7-10], [REPO-TUI-CARGO: crates/tracewake-tui/Cargo.toml lines 7-12].

### 5.4 Migration path

**Stage A — Screen seam first.** Add an internal TUI screen model and headless dump before adding the interactive shell. Introduce:

```rust
pub struct EmbodiedScreenModel { ... }
pub struct ScreenDump { ... }
pub fn build_embodied_screen_model(view: &EmbodiedViewModel, opts: RenderOptions) -> EmbodiedScreenModel;
pub fn render_embodied_to_buffer(model: &EmbodiedScreenModel, area: Rect) -> Buffer;
pub fn render_embodied_screen_dump(model: &EmbodiedScreenModel) -> String;
```

`render_embodied_view(&EmbodiedViewModel) -> String` should remain as a wrapper over the canonical dump, or remain as a compatibility-preserving pure render while the new dump becomes the acceptance target. The crucial rule is that tests and agents can render without a terminal.

**Stage B — Add ratatui renderer under tests.** Add `ratatui` to `tracewake-tui`, implement buffer rendering with fixed terminal sizes, and snapshot the resulting buffer/dump. No crossterm event loop yet.

**Stage C — Add deterministic input intents.** Split input into `UiIntent` values:

```rust
enum UiIntent {
    FocusNext,
    FocusPane(PaneId),
    MoveSelection(Direction),
    ActivateSelection,
    SubmitSemanticAction(SemanticActionId),
    WaitOneTick,
    ContinueUntil { max_ticks: u64 },
    ToggleHelp,
    Quit,
}
```

Keyboard events, script lines, and tests all produce the same intent stream. The intent dispatcher calls existing `TuiApp` methods and runtime commands.

**Stage D — Fullscreen shell.** Add `crossterm` and the normal terminal loop: enter alternate screen/raw mode, read events, translate to `UiIntent`, update UI focus state or submit core commands, redraw from the current view model. The shell must hold only presentation state: focused pane, selected row, scroll offsets, help overlay, last screen size.

**Stage E — Retire or reframe the old line loop.** Do not keep a permanent “old REPL compatibility shim.” Instead, preserve the automation need through a first-class headless/script runner that emits screen dumps and transcripts. Existing command strings can be migrated into scripted intent fixtures only if they remain the declared automation grammar, not as undocumented aliases.

### 5.5 Preserving the pure seam

The new architecture should be:

```text
core event/replay/projection
  -> holder-known context
  -> EmbodiedViewModel / DebugViewModel
  -> tracewake-tui ScreenModel
  -> ratatui Buffer
  -> human terminal OR ScreenDump / snapshot / transcript
```

The interactive shell is not in the middle of this chain. It surrounds it as an input/output adapter. This preserves the settled pure function asset and strengthens it: instead of one text string, Tracewake gets a buffer renderer and a structured dump that can be consumed by humans, tests, and automated agents.

---

## 6. Agent-drivability & testability plan

### 6.1 Requirements

A non-interactive driver must be able to:

1. start a deterministic fixture/session;
2. bind/possess an actor;
3. submit scripted input as keys or semantic intents;
4. advance/wait/continue through the same core authority as a human;
5. capture the rendered screen at fixed sizes;
6. inspect actor-visible text and structured pane contents;
7. compare golden/snapshot output;
8. preserve transcripts with semantic IDs, mode, actor binding, rendered output, and debug markers.

Architecture 10 already requires transcripts to preserve command input, bound actor/mode, semantic IDs, proposal/validation, rendered output, debug markers, and event/replay references, while warning that transcript text alone is not proof [REPO-A10: docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md lines 166-176]. The new plan extends that transcript regime to fullscreen buffers.

### 6.2 Headless render path

Implement two headless artifacts:

**Plain screen dump** — what an agent reads:

```text
SCREEN mode=embodied size=100x30 focus=actors
PANE place title="Bakery kitchen"
  Exits: north shop floor (door closed), east pantry
PANE self
  Hunger severe; Fatigue tired; Intention get food; Routine breakfast
PANE actors
  1. Mara — working at the oven [seen now · visual]
  2. Tomas — activity not apparent [visible]
PANE actions
  1. eat bread semantic=action.eat.bread available
  2. inspect Mara semantic=action.inspect.mara available
  w. wait one tick semantic=action.wait available
PANE recent
  no new actor-known notices or observations
```

**Structured screen dump** — what tests/tools parse:

```rust
struct ScreenDump {
    mode: DumpMode,
    terminal_size: (u16, u16),
    focused_pane: PaneId,
    panes: Vec<PaneDump>,
    action_refs: Vec<RenderedActionRef>,
    debug_marker_present: bool,
    view_model_id: String,
    holder_known_context_hash: String,
}
```

For embodied dumps, metadata may include stable view-model/provenance identifiers already carried by the view model, but it must not include debug-only world truth. Debug dumps are separate and visibly marked.

### 6.3 Deterministic input injection

The driver should support both:

- **Key scripts:** `Tab`, `Down`, `Enter`, `w`, `c`, `?`, etc.
- **Semantic scripts:** `focus actors`, `select actor 1`, `submit semantic_action_id=...`, `wait_one_tick`, `continue_until max_ticks=...`.

The key path proves real human controls. The semantic path gives robust automated scenarios that do not break on harmless keybinding/layout changes. Both paths dispatch through the same `UiIntent` reducer. Neither bypasses validation, proposal construction, runtime receipts, or actor-known projection refresh.

Textual's testing model is useful prior art: its `run_test` and pilot API simulate key presses/clicks and assert resulting UI state; its snapshot docs fix terminal size and emit deterministic snapshots [EXT-TEXTUAL-TESTING], [EXT-TEXTUAL-SNAPSHOT]. Tracewake should implement the Rust/ratatui equivalent using `TestBackend`, intent scripts, and screen dumps.

### 6.4 Golden/snapshot upgrades

Existing tests should evolve as follows:

| Existing obligation | Preserve / upgrade |
|---|---|
| `tui_seam_conformance.rs` | Add screen-model/buffer/dump conformance. Guard that every `EmbodiedViewModel` field and every closed presentation enum has explicit pane disposition. Keep source/compile guards against wildcard/default laundering. |
| `transcript_snapshot.rs` | Snapshot `ScreenDump` plus transcript sections. The transcript should include mode, bound actor, fixed terminal size, key/intent inputs, semantic action ids, rendered pane text, debug markers, and relevant receipt/projection ids. |
| `playable_capability_parity.rs` | Add co-present observed-activity as a declared playable surface: core event/perception -> `VisibleActor.observed_activity` -> rendered actor pane. Include anti-leak cases where truth has activity but actor does not know it. |
| `embodied_flow.rs` | Drive the fullscreen/headless intent loop through ordinary wait/continue/action flows. Prove the full-screen UI does not fork semantics from line commands. |
| TUI goldens | Replace or supplement old linear outputs with fixed-size screen dumps and focused pane snapshots. Keep a text-only dump for agents. |
| Debug/adversarial gates | Assert debug panels are marked non-diegetic and never appear in embodied dumps. Include negative fixtures for activity leakage and debug-to-embodied contamination. |

### 6.5 Non-vacuity requirements

The risk register warns that guards become decorative when they check only table/list/source shape without proving forbidden behavior fails [REPO-RISK: docs/3-reference/01_DESIGN_RISK_REGISTER.md lines 384-392]. Every new TUI guard should therefore include a behavior witness or synthetic negative:

- A mutant/removal of `observed_activity` rendering fails Hop 2.
- A mutant that builds activity from physical truth instead of actor-known context fails anti-contamination tests.
- A hidden actor sleeping behind an occlusion is not rendered in embodied mode.
- A debug activity panel cannot be invoked without debug authority and cannot feed embodied output.
- A stale activity must show stale/uncertain wording; removing the stale label fails.
- A screen-size layout that clips all actors/actions fails snapshot or accessibility checks.

### 6.6 Accessibility and automation constraints

- No color-only semantics. Every warning/status also needs text/symbols.
- Fixed snapshot sizes: at minimum `80x24`, `100x30`, and a narrow size such as `60x20`.
- Stable pane titles and action identifiers in dumps.
- Deterministic ordering of actors/actions/leads.
- Explicit truncation markers with detail expansion, not silent disappearance.
- A headless mode that does not require TTY raw mode.

---

## 7. Ordered spec-decomposition roadmap

Do not assign spec numbers here. The repository ledger owns numbering and placement. The ordering below is intentional: the pure seam and testability land before or alongside fullscreen interactivity.

### Spec A — Screen-model and headless-render seam

- **Objective:** Introduce `EmbodiedScreenModel`, ratatui buffer rendering, plain-text screen dump, and structured `ScreenDump` while preserving `render_embodied_view(&EmbodiedViewModel) -> String` as a pure/headless asset.
- **Scope boundary:** TUI-only. No core view-model changes. No crossterm event loop yet.
- **Dependencies / ordering:** First. This is the foundation for every later UI change.
- **Crate crossing:** TUI-only.
- **Acceptance:** Fixed-size buffer snapshots; `tui_seam_conformance` proves all current `EmbodiedViewModel` fields have pane disposition; debug tokens absent from embodied dumps.

### Spec B — Deterministic UI intent reducer and script driver

- **Objective:** Split input into deterministic `UiIntent` values and add key-script/semantic-script drivers that can run without a TTY.
- **Scope boundary:** TUI-only adapter over existing `TuiApp` and core runtime methods.
- **Dependencies / ordering:** After Spec A; before fullscreen shell.
- **Crate crossing:** TUI-only.
- **Acceptance:** Scripts drive wait, continue, action selection, notebook, and debug authority paths; transcripts include screen dumps and semantic ids.

### Spec C — Core actor-known co-present observed-activity enrichment

- **Objective:** Extend `VisibleActor` / local actor view-model projection with actor-known observed public activity, source, staleness, and uncertainty.
- **Scope boundary:** Core view-model/projection/perception plus fixture support; no UI framework below TUI.
- **Dependencies / ordering:** Can start after Spec A contract is available; must land before the final actor pane is considered complete.
- **Crate crossing:** Crate-crossing (`tracewake-core`, probably `tracewake-content` fixtures, and `tracewake-tui` rendering/tests).
- **Acceptance:** Positive fixtures for visible sleeping/eating/working; negative fixtures for hidden/occluded/unobserved activity; stale-label tests; possession parity; Hop 1 and Hop 2 parity rows.

### Spec D — Embodied pane layout and at-a-glance panels

- **Objective:** Build the fullscreen embodied layout: place, self, actors, actions, details/why-not, notebook/leads, recent interval, input hints.
- **Scope boundary:** TUI-only rendering/focus state; consumes existing and Spec C view-model fields.
- **Dependencies / ordering:** After Spec A; actor panel's final state depends on Spec C.
- **Crate crossing:** TUI-only unless it discovers missing core fields, which must route back to Spec C-style contracts.
- **Acceptance:** Snapshot goldens at fixed sizes; actor-known data only; no debug tokens; pane overflow/truncation behavior tested.

### Spec E — Fullscreen crossterm shell

- **Objective:** Add the real terminal event loop with `crossterm` raw mode/alternate screen and ratatui drawing.
- **Scope boundary:** TUI-only shell adapter. It owns only presentation state.
- **Dependencies / ordering:** After Specs A and B; can be parallel with Spec D after screen model stabilizes.
- **Crate crossing:** TUI-only.
- **Acceptance:** Interactive smoke tests where feasible; headless tests remain authoritative; no TTY needed for CI.

### Spec F — Time-control presentation modernization

- **Objective:** Make wait/continue/time summaries prominent and pleasant while preserving the kernel-owned one-tick coordinator and actor-known interval summary contract.
- **Scope boundary:** Mostly TUI-only; core changes only if new closed time-control view metadata is genuinely required.
- **Dependencies / ordering:** After Specs A/B; after current 0047 behavior remains green.
- **Crate crossing:** Usually TUI-only; possible crate-crossing for explicit time-control view-model metadata.
- **Acceptance:** `wait` and `continue` scripts prove repeated authoritative one-tick progression; interval pane uses actor-known summaries; no raw event diffs in embodied mode.

### Spec G — Conversation panel and structured speech-act UI

- **Objective:** Add a structured conversation surface that renders speech-act choices, topics, evidence attachment, and utterance previews without giving prose authority.
- **Scope boundary:** TUI rendering and possibly architecture/core view-model additions for `ConversationAffordanceView` if current semantic actions are insufficient.
- **Dependencies / ordering:** After Specs A/B/D. Core work should route to architecture 07 and 10.
- **Crate crossing:** Potentially crate-crossing.
- **Acceptance:** LLM-disabled deterministic operation; speech acts submit typed proposals; free text cannot mutate state; actor-known evidence attachment only.

### Spec H — Debug/operator dashboard

- **Objective:** Replace command-emitted debug text with a visibly non-diegetic fullscreen dashboard/tabs.
- **Scope boundary:** TUI-only over existing debug view models and debug authority APIs unless new debug view models are explicitly needed.
- **Dependencies / ordering:** After Specs A/B/E; can proceed independent of Spec C.
- **Crate crossing:** Mostly TUI-only.
- **Acceptance:** Debug marker present on every debug screen; debug unavailable without authority; debug outputs never appear in embodied screen dumps.

### Spec I — Parity registry and acceptance hardening for the new surface

- **Objective:** Update playable-capability registry, Hop 1/Hop 2 evidence, golden snapshots, screen-dump transcripts, and anti-leak negatives for the full overhaul.
- **Scope boundary:** Tests, acceptance artifacts, conformance guards, and possibly reference docs.
- **Dependencies / ordering:** Runs alongside all prior specs; closes after Specs C/D/F/H.
- **Crate crossing:** Crate-crossing because it covers core-to-view-to-render obligations.
- **Acceptance:** Removal of an actor-activity field, pane disposition, stale label, debug marker, or headless dump fails a real test.

### Spec J — Doctrine/reference documentation update

- **Objective:** Document the new view-model contracts, screen-dump acceptance lane, actor-activity source discipline, and risk-register updates.
- **Scope boundary:** Architecture, execution, reference. No foundation amendment unless spec authoring discovers a genuine contradiction.
- **Dependencies / ordering:** Drafted during Specs C/I; finalized after implementation contracts settle.
- **Crate crossing:** Documentation; crate-crossing concepts.
- **Acceptance:** References architecture 10, architecture 03/06, execution 04/07/10, risk register updates; no invented invariant/gate/glossary ids.

---

## 8. Foundational / doctrine amendments (if warranted)

### Determination: no foundational amendment recommended

No amendment to the foundation tier is warranted for this overhaul.

The driver — a sophisticated fullscreen TUI with co-present observed activity — is already authorized by existing doctrine:

- Foundation 08 says embodied mode may show perceived place, exits, nearby actors, objects, traces, affordances, risks, actor-known beliefs/memories/rumors/records, available actions, event summaries, uncertainty/staleness/provenance labels, and why-not explanations; it also forbids hidden truth, hidden actors/routes, other minds, culprit labels, and debug graph data [REPO-F08: docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md lines 53-75].
- Foundation 08 already makes time controls part of the TUI charter and requires actor-known summaries without omniscient leakage [REPO-F08: docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md lines 257-303].
- Foundation 14 already owns the actor-known cognition transaction/truth firewall: truth may validate, but actor-known context controls embodied cognition and presentation [REPO-F14: docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md].
- Foundation 04 already defines legitimate information-flow sources — perception, memory, inference, testimony, and records — which are sufficient to explain co-present activity labels [REPO-F04: docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md].
- Architecture 10 already has `visible_actors`, two-hop presentation disposition, captured-projection rules, future-client parity, and transcript obligations [REPO-A10: docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md lines 37-86, 166-188].

The overhaul therefore needs architecture/execution/spec work, not constitutional change. The target docs that must own new substance are listed in §4.3 and §7 Spec J. Do not mint new invariant identifiers, gate codes, or glossary terms for this report.

### Doctrine updates below foundation

Although no foundation amendment is recommended, the following lower-tier updates are warranted:

- **Architecture 10:** expand the embodied view-model contract from visible actor identity to visible actor awareness; define screen model / headless dump as client rendering artifacts; keep menu positions presentation-only and closed disposition mandatory.
- **Architecture 03:** explicitly include co-present public activity observations as holder-known context material when captured by modeled observation/provenance.
- **Architecture 06:** define public activity observation flow into observations/beliefs/memories/leads.
- **Architecture 13 / Execution 10:** recognize screen dumps and buffer snapshots as review artifacts, with non-vacuity requirements.
- **Reference risk register:** add or update risks for fullscreen UI swallowing the pure seam, actor-activity truth leakage, and agent automation regression; cross-reference existing TUI parity/debug contamination risks.

---

## 9. Open questions & risks

### 9.1 Open questions for spec authoring

1. **Activity taxonomy granularity.** Should `ObservedActorActivityKind` be coarse (`Sleeping`, `Eating`, `Working`, `Moving`, `Speaking`, `Waiting`) or include domain-specific subtypes? Recommendation: start coarse and attach actor-safe summary text for domain flavor.
2. **Staleness labels.** Should freshness be exact tick metadata, actor-facing labels, or both? Recommendation: core carries enough provenance for tests; embodied UI displays actor-relevant labels, not raw debug time unless the actor-known time model supports it.
3. **Identity uncertainty.** Can the current epistemic model represent “someone nearby” separate from a known `ActorId`? If not, do not fake it; start with identified visible actors and add uncertain-presence rows only when core can model them.
4. **Conversation scope.** Are conversation affordances already adequately represented by semantic actions, or is a new `ConversationAffordanceView` needed? Route to architecture 07/10 if needed.
5. **Default launch behavior.** When should fullscreen become default? Recommendation: only after the headless screen dump and intent driver are green and CI-stable.
6. **Snapshot churn budget.** Ratatui layout changes can produce large snapshot diffs. The spec should define stable pane dumps as the primary golden and raw buffers as secondary visual review artifacts.
7. **Terminal size floor.** Decide the minimum supported size. Recommendation: hard fail with an actor-safe message below a floor, and snapshot `80x24`, `100x30`, and a narrow fallback.
8. **Color/theme policy.** Decide whether themes are allowed. Recommendation: yes, TUI-only, but all semantics must be present in text.

### 9.2 Risks introduced

| Risk | Why it matters | Mitigation |
|---|---|---|
| Fullscreen shell swallows the pure seam | The current pure renderer is a major testability asset. | Land screen model/headless dump first; event loop is adapter only. |
| Co-present activity leaks truth | The feature's product value is exactly where hidden-truth temptation is strongest. | Core-only actor-known activity projection; anti-leak fixtures; stale/uncertain labels; no TUI physical-state handle. |
| Activity labels become prose-born facts | “Mara is working” could be laundered through display strings. | Closed activity enums plus actor-safe summaries with provenance; no freeform TUI derivation. |
| Snapshot tests become brittle decoration | Fullscreen buffers can churn without proving behavior. | Pair buffer snapshots with structured pane dumps and synthetic negatives, per risk-register non-vacuity guidance [REPO-RISK: docs/3-reference/01_DESIGN_RISK_REGISTER.md lines 384-392]. |
| Agents lose ability to play | Raw-mode fullscreen UIs often break stdin/stdout drivers. | First-class headless script/intent runner; fixed-size `TestBackend`; structured screen dump. |
| Debug truth visually bleeds into embodied mode | More panes increase leakage risk. | Permanent mode split, debug markers, authority checks, negative tests, no shared debug/embodied pane components unless marker policy is enforced. |
| Pane overload hides critical data | Multi-pane UIs can bury information. | Stable priority ordering, truncation markers, detail focus, minimum terminal size, no silent omission of safety/why-not/action information. |

### 9.3 Risks retired or reduced

- **TUI-first playability erosion / renderer blindness.** The overhaul directly strengthens the risk register's TUI parity concern by making every capability surface pane-disposed and headless-inspectable [REPO-RISK: docs/3-reference/01_DESIGN_RISK_REGISTER.md lines 214-219].
- **Opaque time passage.** Time controls become visible controls with actor-known interval summaries, reinforcing 0047 rather than relitigating it.
- **Co-present blindness.** Visible actors stop being id-only rows and become actor-known awareness cards.
- **Debug/embodied confusion.** Debug becomes more powerful but more visibly separated.

---

## 10. References

### 10.1 External sources

[EXT-RATATUI-BACKENDS] Ratatui, “Backends.” <https://ratatui.rs/concepts/backends/>. Cited for ratatui backend abstraction and terminal drawing responsibilities.

[EXT-RATATUI-TESTBACKEND] docs.rs, `ratatui::backend::TestBackend`. <https://docs.rs/ratatui/latest/ratatui/backend/struct.TestBackend.html>. Cited for in-memory buffer integration testing.

[EXT-RATATUI-SNAPSHOTS] Ratatui, “Testing / Widgets with Insta.” <https://ratatui.rs/recipes/testing/widgets-with-insta/>. Cited for ratatui snapshot testing practice.

[EXT-RATATUI-FAQ] Ratatui, “FAQ.” <https://ratatui.rs/faq/>. Cited for ratatui event-loop/redraw model and alternatives discussion.

[EXT-CROSSTERM] docs.rs, `crossterm` crate documentation. <https://docs.rs/crossterm/latest/crossterm/>. Cited for pure-Rust, cross-platform terminal manipulation.

[EXT-CROSSTERM-EVENT] docs.rs, `crossterm::event` module documentation. <https://docs.rs/crossterm/latest/crossterm/event/index.html>. Cited for keyboard/mouse/resize polling and raw-mode event behavior.

[EXT-TEXTUAL-TESTING] Textual documentation, “Testing.” <https://textual.textualize.io/guide/testing/>. Cited as prior art for headless TUI testing and simulated key presses.

[EXT-TEXTUAL-SNAPSHOT] Textual documentation, “Snapshot Testing.” <https://textual.textualize.io/guide/snapshot_testing/>. Cited as prior art for fixed-size UI snapshots.

[EXT-NNG-STATUS] Nielsen Norman Group, “Visibility of System Status.” <https://www.nngroup.com/articles/visibility-system-status/>. Cited for system-status visibility as a usability principle.

[EXT-NNG-RECOGNITION] Nielsen Norman Group, “Recognition Rather than Recall.” <https://www.nngroup.com/articles/recognition-and-recall/>. Cited for visible actions/options and reduced memory load.

[EXT-ROGUEBASIN-FOV] RogueBasin, “Field of Vision.” <https://chizaruu.github.io/roguebasin/field_of_vision>. Cited for perception-gated roguelike visibility precedent.

[EXT-COGMIND-FOV] Cogmind / Grid Sage Games, “Field of Vision.” <https://www.gridsagegames.com/blog/2014/12/field-vision/>. Cited for line-of-sight/opaque-wall entity visibility precedent.

[EXT-HEXWORKS-FOV] Hexworks Zircon, “How to make a roguelike: Vision and Fog of War.” <https://hexworks.org/posts/tutorials/2019/04/27/how-to-make-a-roguelike-vision-and-fog-of-war.html>. Cited for component-driven vision/fog-of-war implementation precedent.

[EXT-DF-VIEW-JOB] Dwarf Fortress Wiki, “v0.34:Status icon / View job.” <https://dwarffortresswiki.org/index.php/v0.34:Status_icon>. Cited as simulation-UI prior art for creature jobs/statuses as useful surface information, not as doctrine.

[EXT-CURSIVE] docs.rs, `cursive` crate documentation. <https://docs.rs/cursive/latest/cursive/>. Cited for alternative higher-level TUI framework posture.

[EXT-TUIREALM] `tui-realm` project documentation / repository. <https://github.com/veeso/tui-realm>. Cited for alternative ratatui-based framework posture.

[EXT-TERMION] docs.rs, `termion` crate documentation. <https://docs.rs/termion/latest/termion/>. Cited for lower-level terminal manipulation alternative.

### 10.2 Repository source citations used in analysis

All repository sources below were fetched from exact target-commit raw URLs listed in §10.3. They are not branch, default-branch, search, or snippet evidence.

- [REPO-README] `docs/README.md` — authority order and tier layering, especially lines 3-15 and 43-57.
- [REPO-F02] `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` — constitutional TUI/truth-firewall invariants, especially lines 43 and 291-301.
- [REPO-F04] `docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md` — legitimate information sources and actor-known information flow.
- [REPO-F08] `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` — TUI charter, embodied/debug split, view-model boundary, time controls, and acceptance posture, especially lines 5-10, 53-132, 257-303, and 339-353.
- [REPO-F11] `docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md` — structured speech acts and language boundary.
- [REPO-F14] `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` — actor-known transaction and truth firewall.
- [REPO-A01] `docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md` — crate/dependency and TUI boundary posture.
- [REPO-A03] `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` — holder-known context, provenance, truth firewall.
- [REPO-A06] `docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md` — observation/belief/memory flow.
- [REPO-A10] `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` — embodied view-model schema, two-hop contract, captured-projection rule, time controls, why-not, debug split, transcript obligations, and future-client parity, especially lines 37-188.
- [REPO-A13] `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md` — review-artifact and acceptance posture.
- [REPO-E04] `docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md` — anti-contamination gates.
- [REPO-E07] `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` — embodied/debug view-model proof obligations.
- [REPO-E10] `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — golden/snapshot/testing and review artifacts.
- [REPO-RISK] `docs/3-reference/01_DESIGN_RISK_REGISTER.md` — TUI parity/debug/truth leakage/non-vacuity risks, especially lines 214-219 and 384-392.
- [REPO-GLOSSARY] `docs/3-reference/02_GLOSSARY.md` — canonical terminology.
- [REPO-LEDGER] `docs/4-specs/SPEC_LEDGER.md` — source discipline and recent spec history, especially lines 17-23 and 101-103.
- [REPO-0046] `archive/specs/0046_TUI_SIMULATION_PLAYABLE_CAPABILITY_PARITY_CONTRACT_AND_TWO_HOP_DRIFT_GUARD_HARDENING_SPEC.md` — two-hop playable-capability parity contract, especially lines 31-67.
- [REPO-0047] `archive/specs/0047_TUI_AUTHORITATIVE_WORLD_ADVANCE_DURATION_COMPLETION_AND_ACTOR_KNOWN_INTERVAL_SUMMARIES_SPEC.md` — kernel-owned one-tick coordinator and actor-known interval summary contract, especially lines 55-104.
- [REPO-0057] `archive/specs/0057_EMBODIED_ROUTINE_CONTINUATION_BEHAVIORAL_PROGRESS_AND_POSSESSION_PARITY_SPEC.md` — embodied routine continuation behavior.
- [REPO-0058] `archive/specs/0058_EMBODIED_ROUTINE_CONTINUATION_FOUNDATIONAL_ALIGNMENT_HARDENING_SPEC.md` — embodied routine continuation foundational alignment.
- [REPO-0059] `archive/specs/0059_AUTONOMOUS_SCHEDULER_ROUTINE_DERIVATION_ACTIVE_INTENTION_AUTHORITY_HARDENING_SPEC.md` — active-intention routine derivation context.
- [REPO-PARITY-REPORTS] `reports/tui-parity.md`, `reports/tui-parity-research-brief.md`, `reports/tui-parity-research-report.md` — predecessor parity context.
- [REPO-WAIT-REPORTS] `reports/tui-human-wait-runs-simulation-issue.md`, `reports/tui-human-wait-runs-simulation-research-brief.md`, `reports/tui-human-wait-runs-simulation-research-report.md` — predecessor time-control context.
- [REPO-RUN] `crates/tracewake-tui/src/run.rs` — current line command loop, especially lines 1-31 and 34-80.
- [REPO-INPUT] `crates/tracewake-tui/src/input.rs` — current text command parser, especially lines 4-103.
- [REPO-RENDER] `crates/tracewake-tui/src/render.rs` — pure embodied renderer and current actor rendering, especially lines 5-20 and 127-130.
- [REPO-APP] `crates/tracewake-tui/src/app.rs` — app state and render invocation, especially lines 56-64 and 201-203.
- [REPO-DEBUG-PANELS] `crates/tracewake-tui/src/debug_panels.rs` — current debug panel renderers.
- [REPO-TRANSCRIPT] `crates/tracewake-tui/src/transcript.rs` — transcript capture.
- [REPO-VM] `crates/tracewake-core/src/view_models.rs` — `EmbodiedViewModel`, `VisibleActor`, notebook, and related view-model types, especially lines 18-42, 337-350, and 389-392.
- [REPO-PERCEPTION] `crates/tracewake-core/src/agent/perception.rs` — actor-known context construction and local actor facts, especially lines 265-283 and 517-529.
- [REPO-TUI-CARGO] `crates/tracewake-tui/Cargo.toml` — TUI crate dependencies, especially lines 7-12.
- [REPO-CORE-CARGO] `crates/tracewake-core/Cargo.toml` — zero normal dependency posture, especially lines 7-10.
- [REPO-TUI-TESTS] `crates/tracewake-tui/tests/tui_seam_conformance.rs`, `transcript_snapshot.rs`, `playable_capability_parity.rs`, `embodied_flow.rs`, `command_loop_session.rs`, `tui_acceptance.rs`, `adversarial_gates.rs`, `parity_adversarial.rs`, and `tests/goldens/*` — acceptance regime to preserve and upgrade.

### 10.3 Complete repository acquisition ledger

```text
Requested repository: joeloverbeck/tracewake
Target commit: 85dd8836508a58305eb84d70caf52bda088a9bde
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: web.run.open with full raw.githubusercontent.com exact-commit URLs
Requested file count: 125
Successfully verified file count: 125
Fetched repository files:
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/0-foundation/00_FOUNDATION_INDEX.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/0-foundation/01_PROJECT_CHARTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/0-foundation/04_CLAIMS_BELIEFS_MEMORY_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/0-foundation/05_AGENTS_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/0-foundation/07_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/0-foundation/10_SCALE_LOD_REGIONAL_BOUNDARIES_AND_LONG_SIMULATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/0-foundation/11_LLM_SPEECH_ACTS_AND_LANGUAGE_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/0-foundation/13_RESEARCH_DECISIONS_AND_SOURCE_NOTES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/1-architecture/05_ACTOR_DECISION_TRANSACTION_NEEDS_INTENTIONS_ROUTINES_AND_PLANNING.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/1-architecture/06_CLAIMS_BELIEFS_OBSERVATION_MEMORY_TRACES_AND_INFORMATION_FLOW.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/1-architecture/07_SPEECH_ACTS_LANGUAGE_SURFACES_AND_LLM_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/1-architecture/08_INSTITUTIONS_HOUSEHOLDS_NORMS_RECORDS_AND_PROCEDURES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/1-architecture/11_INCIDENTS_LEADS_NOTICES_AND_STORY_SIFTING_PROJECTIONS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/1-architecture/12_LOD_REGIONAL_PROCESSES_PREHISTORY_AND_SCALE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/1-architecture/14_RESEARCH_DECISIONS_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/2-execution/00_EXECUTION_INDEX_AND_AUTHORITY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/2-execution/01_POST_0008_BASELINE_ARCHIVED_SPEC_STATUS_AND_CODE_AUDIT_BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/2-execution/02_FIRST_PROOF_SCOPE_CURRENT_BASELINE_AND_ACCEPTANCE_CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/2-execution/03_PHASE_LADDER_GATE_ORDER_AND_CERTIFICATION_SEQUENCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/2-execution/04_TRUTH_FIREWALL_ACTOR_KNOWN_AND_ANTI_CONTAMINATION_GATES.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/2-execution/11_INSTITUTIONS_RECORDS_WRONG_SUSPICION_AND_PHASE_4_ENTRY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/2-execution/12_DEFERRED_SECOND_PROOF_NOTICES_TRAVEL_REGIONAL_SCALE_AND_LOD.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/2-execution/13_RESEARCH_DECISIONS_SOURCE_NOTES_AND_FORBIDDEN_MISREADS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/3-reference/00_REFERENCE_INDEX_AND_REVIEW_CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/3-reference/01_DESIGN_RISK_REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/3-reference/02_GLOSSARY.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/4-specs/0001_MISSING_PROPERTY_VILLAGE_ONTOLOGY_AND_FIXTURE_CONTRACTS.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/4-specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/docs/4-specs/SPEC_LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/reports/tui-parity.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/reports/tui-parity-research-brief.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/reports/tui-parity-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/reports/tui-human-wait-runs-simulation-issue.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/reports/tui-human-wait-runs-simulation-research-report.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/reports/tui-human-wait-runs-simulation-research-brief.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/archive/specs/0046_TUI_SIMULATION_PLAYABLE_CAPABILITY_PARITY_CONTRACT_AND_TWO_HOP_DRIFT_GUARD_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/archive/specs/0047_TUI_AUTHORITATIVE_WORLD_ADVANCE_DURATION_COMPLETION_AND_ACTOR_KNOWN_INTERVAL_SUMMARIES_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/archive/specs/0057_EMBODIED_ROUTINE_CONTINUATION_BEHAVIORAL_PROGRESS_AND_POSSESSION_PARITY_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/archive/specs/0058_EMBODIED_ROUTINE_CONTINUATION_FOUNDATIONAL_ALIGNMENT_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/archive/specs/0059_AUTONOMOUS_SCHEDULER_ROUTINE_DERIVATION_ACTIVE_INTENTION_AUTHORITY_HARDENING_SPEC.md
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-core/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/src/main.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/src/launch.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/src/run.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/src/render.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/src/app.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/src/input.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/src/debug_panels.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/src/transcript.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-core/src/view_models.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-core/src/projections.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-core/src/agent/perception.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-core/src/agent/actor_known.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-core/src/agent/intention.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-core/src/agent/routine_continuation.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-core/src/epistemics/observation.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-core/src/epistemics/projection.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-core/src/epistemics/knowledge_context.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-core/src/runtime/command.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-core/src/runtime/session.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-core/src/runtime/receipt.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-core/src/controller.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-core/src/scheduler.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-core/src/actions/report.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-core/src/actions/proposal.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/tui_seam_conformance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/transcript_snapshot.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/playable_capability_parity.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/embodied_flow.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/command_loop_session.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/tui_acceptance.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/adversarial_gates.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/parity_adversarial.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/parity/mod.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/parity/runner.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/parity/scenario.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/parity/census_actions.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/parity/census_families.rs
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/action_check_container.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/action_close.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/action_continue_routine.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/action_eat.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/action_inspect_entity.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/action_inspect_place.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/action_look.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/action_open.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/action_place.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/action_sleep.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/action_take.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/action_truthful_accuse_probe.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/action_work_block.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/base_epistemic_why_not_door_closed.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/base_semantic_action_wait.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/family_debug_quarantine.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/family_epistemic_filtering.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/family_needs_routines.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/family_no_human_autonomy.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/family_notebook_leads.txt
- https://raw.githubusercontent.com/joeloverbeck/tracewake/85dd8836508a58305eb84d70caf52bda088a9bde/crates/tracewake-tui/tests/goldens/family_rejection_why_not.txt
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```
