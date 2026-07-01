# 0062 TUI Deterministic UI Intent Reducer and Script Driver Spec

**Status**: COMPLETED

This is a post-`FIRST-PROOF-CERT` **feature/capability** spec in the parallel `specs/NNNN`
series. It is staged in `specs/` and promoted to `archive/specs/` on acceptance; it is never
promoted to the live `docs/4-specs/` tier. It is not a certification audit, mutation-remediation
pass, Phase-4 entry claim, or latest-`main` certification. It uses the sibling hardening-spec
house structure (`0046`, `0047`, `0057`).

This is **Spec B** of the TUI Experience Overhaul roadmap
(`reports/tui-experience-overhaul-research-report.md` §7). It splits input into deterministic
`UiIntent` values and adds key-script and semantic-script drivers that run **without a TTY**, so
golden tests and scripted agents drive the same authority a human does.

## 0. Baseline statement and source discipline

- **Driver.** The TUI Experience Overhaul research report §5.4 (Stage C) and §6 (agent-drivability
  as a co-equal product requirement). Analysis only; mints no doctrine.
- **Baseline commit.** Re-verified against repository `HEAD`
  `85dd8836508a58305eb84d70caf52bda088a9bde`. Named symbols are authoritative; line numbers are
  provenance only.
- **Source discipline.** Commit hashes are audit/provenance only. No latest-`main` certification,
  Phase-4 entry, or second-proof entry is declared. A manifest is path inventory only.
- **Authority posture.** Subordinate to foundation → architecture → execution → reference. Routes
  any doctrine sharpening as *substance + home* (§5), ratifying no wording and minting no id.
- **Execution admissibility posture.** `P0-CERT not applicable`.
- **Ledger timing.** Archived-row entry at acceptance/closeout, not at proposal. No ledger row now.

## 1. Scope

### 1.1 In scope

In `tracewake-tui` only, replace ad-hoc text-command dispatch with a deterministic intent layer:

1. **`UiIntent` enum** — a closed set of presentation/command intents, e.g. `FocusNext`,
   `FocusPane(FocusedPane)`, `MoveSelection(Direction)`, `ActivateSelection`,
   `SubmitSemanticAction(SemanticActionId)`, `SubmitDebugCommand(DebugCommand)`, `WaitOneTick`,
   `ContinueUntil { max_ticks }`, `ToggleHelp`, `FocusNotebook`, `Quit`. `FocusedPane` reuses the
   landed Spec `0061` enum (`crates/tracewake-tui/src/screen/model.rs`); `DebugCommand` reuses the
   existing `crates/tracewake-tui/src/input.rs` type.
2. **Intent reducer** — a single dispatcher mapping each `UiIntent` either to a mutation of the
   presentation-state model (item 6) or to an already-authorized `TuiApp`/core-runtime call —
   including the existing debug-authority path for `SubmitDebugCommand`. It reuses
   `TuiApp::submit_semantic_action` and the settled kernel-owned time-control path; it
   creates **no** new world-mutation path and bypasses no validation.
3. **Key-script driver** — parses a script of key tokens (`Tab`, `Down`, `Enter`, `w`, `c`, `?`,
   `n`, `q`, …) into `UiIntent` values.
4. **Semantic-script driver** — parses a script of semantic lines (`focus actors`, `select actor 1`,
   `submit semantic_action_id=…`, `wait_one_tick`, `continue_until max_ticks=…`) into the same
   `UiIntent` stream.
5. **Headless runner** — runs either script against a `TuiApp` with no raw-mode TTY, emitting the
   screen dumps from Spec `0061` plus a transcript that includes mode, bound actor, terminal size,
   key/intent inputs, semantic action ids, rendered pane text, and debug markers.
6. **Presentation-state model** — a minimal TUI-only struct the reducer mutates and the headless
   runner renders and dumps: the focused pane (reusing Spec `0061`'s landed `FocusedPane`), a
   per-pane selection index, scroll offsets, and a help-overlay flag. Presentation-only; no core
   types and no world authority. Spec `0062` defines it so the headless runner is self-testing
   before any interactive shell exists; Spec `0065`'s fullscreen shell later *holds and drives this
   same model* and introduces no new presentation-state type.

### 1.2 Out of scope (non-goals)

- No `crossterm` event loop (Spec `0065`); no key *events* are read from a terminal here — only
  scripted key *tokens*.
- No new core view-model fields and no new core types; the reducer consumes only existing
  `TuiApp`/runtime surfaces for world changes. The presentation-state model (§1.1 item 6) is new
  TUI-only presentation state, not a world/core surface.
- No change to semantic-action semantics or the 0047 time-control contract; this spec routes to
  them, it does not redefine them.
- The legacy line grammar is not preserved as an undocumented alias; if migrated, it becomes a
  declared scripted-intent grammar (report §5.4 Stage E).

## 2. Dependencies and ordering

- **Depends on:** `0061` (drivers render through the screen-model/dump seam).
- **Blocks:** `0065` (the shell translates real key events into this same `UiIntent` stream),
  `0066` (time-control scripts), `0067`, `0068`, and `0069` (script-driven acceptance).
- **Contract to `0065`:** the presentation-state model (§1.1 item 6) is the state `0065`'s
  fullscreen shell holds and drives; `0065` introduces no new presentation-state type.
- **Parallelizable with:** `0063`, `0064` after `0061` stabilizes.

## 3. Doctrine anchors

- **Architecture 10** (`docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`):
  transcripts preserve command input, bound actor/mode, semantic ids,
  proposal/validation, rendered output, debug markers, and event/replay refs — while transcript
  text alone is not proof. The intent regime extends this to scripted drivers.
- **Architecture 04** (`docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`)
  and **Execution 05** (`…/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`): no
  direct dispatch; every world change goes through proposal/validation/commitment.
- **Foundation 08** (`docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`): possession is
  ordinary; the automation path uses the same authority as a human.

## 4. Findings and remediation requirements

- **4.1 One intent stream, three producers (seam: reducer).** Keyboard events (Spec `0065`), key
  scripts, and semantic scripts all produce the identical `UiIntent` stream through one reducer, so
  automated scenarios and human play cannot fork semantics.
- **4.2 No intent mutates world state directly (seam: `SubmitSemanticAction` /
  `WaitOneTick`/`ContinueUntil`).** World-advancing intents call existing authorized methods; they
  never construct or apply events themselves.
- **4.3 Key path proves human controls; semantic path is layout-robust (seam: drivers).** The key
  path exercises real bindings; the semantic path survives harmless keybinding/layout changes.
  Both are required.
- **4.4 Headless without TTY (seam: runner).** The runner must operate with no raw mode, so CI needs
  no terminal.
- **4.5 Debug authority routes through the same reducer (seam: reducer).** Debug-authority commands
  (the existing `DebugCommand` surface) enter as `SubmitDebugCommand` intents so scripted and human
  debug access cannot fork from the one intent stream (§4.1); the reducer dispatches them through the
  existing debug-authority path, which stays non-diegetic and authority-gated.
- **4.6 Implementation decomposition.** `UiIntent` enum + reducer; presentation-state model
  (focused pane / selection / scroll / help overlay); key-script parser; semantic-script parser;
  debug-authority routing (`SubmitDebugCommand` → existing debug path); headless runner + transcript
  emission; conformance test binding all three producers to one reducer. Separate tickets.

## 5. Doctrine amendments (routed as substance + home; not ratified here)

- **Architecture 10** — extend the transcript-obligation clause to cover scripted key/intent drivers
  and fullscreen dumps (shared with Spec `0070`). Substance only.

## 6. Required fixtures and tests

- A key script and an equivalent semantic script that produce byte-identical screen dumps.
- A script exercising wait, continue, focus/selection movement, action selection, and notebook focus;
  assert resulting dumps (including focused pane and selection) and transcript sections.
- A script that drives a debug-authority command (e.g. `overlay`, `log`) through `SubmitDebugCommand`;
  assert the debug marker is present and that debug output never appears in an embodied dump.
- Negative: a script that attempts a disallowed direct mutation has no code path (compile/source
  guard that intents route only through authorized methods).
- Determinism: same script → same transcript and dumps across runs.

## 7. Acceptance artifact and evidence

A review artifact recording the key-vs-semantic dump equivalence, the driven wait/continue/action/
notebook transcript, and the no-direct-dispatch guard result, at the exact implementation commit.

## 8. Implementation constraints

- TUI-only adapter over existing `TuiApp` and core runtime methods.
- No `crossterm`. No new core types.
- Deterministic parse: unknown tokens fail closed with an explicit error, never silently ignored.

## 9. Risks and open questions

- **Risk: agents lose the ability to play** if the shell later assumes a TTY. Mitigation: the
  headless runner is first-class and CI-authoritative before Spec `0065` lands.
- **Open question:** exact semantic-script grammar surface — kept minimal and closed; extended only
  when a driven scenario needs it.

## 10. Invariants alignment

| Invariant | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| INV-069 (TUI must not implement simulation rules) | aligns | Intents route to authorized methods; no world mutation in the reducer. |
| INV-104 (routines/needs don't dispatch primitives directly) | aligns | No-direct-dispatch preserved; intents go through the pipeline. |
| INV-108 (possession is cognition-neutral) | aligns | Scripted drivers use the same authority/input surface as a human. |
| INV-095 (TUI/view-model tests are acceptance tests) | aligns | Script-driven dumps/transcripts are acceptance evidence. |
| INV-018 (deterministic replay is foundational) | aligns | World-advancing intents route through the kernel, so the event log and its replay are unchanged; deterministic intent parsing separately yields byte-stable dumps/transcripts (a testing property, distinct from the replay contract). |

## Outcome

Completed: 2026-07-01

Implemented the scoped Spec 0062 deterministic TUI intent package in dependency
order:

- `0062TUIDETUI-001` added the TUI-only `UiIntent` enum and `PresentationState`
  model.
- `0062TUIDETUI-002` added the single reducer over presentation mutation,
  authorized semantic submission, authorized time control, debug rendering, and
  quit outcomes.
- `0062TUIDETUI-003` added the deterministic fail-closed key-script parser.
- `0062TUIDETUI-004` added the deterministic fail-closed semantic-script parser.
- `0062TUIDETUI-005` added the terminal-free headless runner and additive
  scripted transcript sections.
- `0062TUIDETUI-006` added the intent conformance capstone proving key/semantic
  dump equivalence, driven flow coverage, debug separation, no-direct-dispatch
  source guard, and byte-stable reruns.

Commit roles:

- Spec correction and implementation baseline commits: `06ebebd`, `c1d4823`,
  `2f4c3e0`, `941a1dd`, `6120eba`, and `578ee98`.
- Acceptance report / archive-truthing commit: this commit moving the report and
  spec and repairing references.

Acceptance artifact:

- `archive/reports/0062_tui_deterministic_intent_acceptance.md`.

Verification before report closeout:

- `cargo test -p tracewake-tui --test intent_conformance` passed.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo build --workspace --all-targets --locked` passed.
- `cargo test --workspace` passed.

Final closeout gates are rerun after this report, the spec, and ledger are
archived/truthed.

This is a scoped TUI feature/capability acceptance. It does not certify latest
main, Phase-4 entry, second-proof entry, institutions, notices, travel, LOD,
LLM/speech, story-sifting, `P0-CERT`, `FIRST-PROOF-CERT`, or any whole-project
status, and it mints no invariant, gate, glossary term, or risk ID.
