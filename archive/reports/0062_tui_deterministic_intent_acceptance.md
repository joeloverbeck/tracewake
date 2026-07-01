# 0062 TUI Deterministic Intent Acceptance

**Status**: ACCEPTED

## Scope

This artifact records the scoped Spec 0062 deterministic TUI intent reducer and
script-driver evidence. It is not a certification audit, latest-main
certification, Phase-4 entry claim, second-proof entry claim, or doctrine
amendment.

Implementation baseline commits:

- `06ebebd` - `UiIntent` and `PresentationState`.
- `c1d4823` - intent reducer.
- `2f4c3e0` - key-script parser.
- `941a1dd` - semantic-script parser.
- `6120eba` - headless runner and scripted transcript sections.
- `578ee98` - intent conformance capstone test.

Evidence/report commit role: the closeout commit that archives this report and
the governing spec after the final closeout gates run.

## Evidence Items

### One Intent Stream

`crates/tracewake-tui/tests/intent_conformance.rs` proves that a key script and
an equivalent semantic script produce byte-identical screen dumps through the
shared headless runner and reducer:

- key script: `Tab w n`
- semantic script: `focus exits`, `wait_one_tick`, `notebook`

Both paths drive the same `UiIntent` reducer and render through the Spec 0061
`EmbodiedScreenModel` / text-dump seam.

### Driven Flow

The capstone drives:

- focus movement through `focus actions`;
- selection movement through `move down`;
- action submission through `submit semantic_action_id=open.container.strongbox_tomas`;
- time control through `wait_one_tick` and `continue_until max_ticks=1`;
- notebook focus through `notebook`.

The transcript sections include input source, raw input, intent text, semantic
action id when present, mode, bound actor, terminal size, focused pane, debug
marker presence, separate debug output, and rendered pane text.

### Debug Authority

The capstone drives `UiIntent::SubmitDebugCommand(DebugCommand::Overlay)`
directly through the reducer. The reducer returns marked debug output while the
subsequent embodied screen dump remains free of `DEBUG NON-DIEGETIC`.

The debug command is driven directly as an intent because the key-script and
semantic-script grammars intentionally remain minimal and closed in Spec 0062.

### No Direct Dispatch

The capstone source guard checks `crates/tracewake-tui/src/intent/reducer.rs`
for forbidden direct world-mutation tokens (`RuntimeCommand::`,
`.submit_command(`, event append/apply tokens) and required authorized seams
(`submit_semantic_action`, `advance_until`, `run::render_debug`).

### Determinism

The capstone reruns the same semantic script from a fresh fixture and asserts
byte-identical dumps and scripted transcript text.

## Verification

Commands run on 2026-07-01 before this report file existed:

- `cargo test -p tracewake-tui --test intent_conformance` passed.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo build --workspace --all-targets --locked` passed.
- `cargo test --workspace` passed.

The spec closeout reruns the repository final gates after this report, the spec,
and ledger have been archived/truthed.

## Verdict

Spec 0062 deterministic TUI intents are accepted as a scoped feature/capability
package. The evidence proves deterministic key and semantic script parsing,
single-reducer routing, terminal-free headless execution, scripted transcript
emission, debug-authority separation, no direct world mutation in the reducer,
and byte-stable dump/transcript behavior for the covered fixtures.

This verdict does not certify latest main, Phase-4 entry, second-proof entry,
institutions, notices, travel, LOD, LLM/speech, story-sifting, `P0-CERT`,
`FIRST-PROOF-CERT`, or any whole-project status. It mints no invariant, gate,
glossary term, or risk ID.
