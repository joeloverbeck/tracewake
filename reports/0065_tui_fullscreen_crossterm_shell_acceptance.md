# 0065 TUI Fullscreen Crossterm Shell Acceptance

Status: accepted for ticket evidence closeout on 2026-07-02.

This artifact records scoped evidence for Spec 0065, the fullscreen crossterm
shell feature line. It is observer-only, non-diegetic evidence: it adds no
simulation fact, no doctrine, no new gate, and no player-only action path.
Headless/no-TTY tests are the authoritative acceptance lane.

## Commit Roles

- Implementation commits:
  - `43e06fa` `0065TUIFULCRO-001` crossterm dependency, terminal lifecycle, and
    restore guard.
  - `cd6f162` `0065TUIFULCRO-002` key-event mapping, shell state, event loop,
    and buffer redraw seam.
  - `1df5719` `0065TUIFULCRO-003` fullscreen launch option, line-loop
    reframing, and selected-action activation parity.
- Evidence/report commit: the commit that adds this report, updates the
  source/dependency guard truthing for the new TUI shell files, and archives
  `0065TUIFULCRO-004`.
- Archive/truthing commit: the later closeout commit that moves this report to
  `archive/reports/0065_tui_fullscreen_crossterm_shell_acceptance.md`, archives
  the spec, and adds the ledger row.

## Gate Evidence

After implementation and guard truthing, the following commands passed:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

The initial `cargo test --workspace` run after the implementation commits
failed only because `workspace_source_classification_census_matches_production_tree`
and `workspace_dependency_posture_matches_allowlist` had not yet been updated
for `crates/tracewake-tui/src/shell/*.rs` and `crossterm`. The guard was updated
to classify the new shell files under the existing TUI presentation-layer
rationale and to allow `crossterm` as a TUI dependency; then
`cargo test -p tracewake-core --test anti_regression_guards workspace_` passed.
The full gate sequence above was rerun after that update and passed.

## Witness Matrix

| Spec 0065 obligation | Behavior witness | Status |
|---|---|---|
| Terminal raw-mode / alternate-screen lifecycle restores on normal exit | `shell::terminal::tests::terminal_guard_restores_on_normal_drop` | Pass |
| Terminal restore also occurs during panic unwind | `shell::terminal::tests::terminal_guard_restores_during_panic_unwind` | Pass |
| Live crossterm keys produce the same intent stream as Spec 0062 key scripts | `shell::key_map::tests::crossterm_keys_match_script_intents_for_equivalent_keys` | Pass |
| Non-press and unknown keys do not mint unauthorized intents | `shell::key_map::tests::release_and_unknown_keys_are_ignored` | Pass |
| Redraw uses the Spec 0064 screen model and buffer renderer without drift | `shell::event_loop::tests::redraw_matches_direct_screen_buffer_rendering` | Pass |
| Shell buffer output is no-TTY testable | `shell::event_loop::tests::draw_buffer_writes_plain_buffer_without_tty` | Pass |
| Quit intent terminates the shell key step | `shell::event_loop::tests::quit_intent_terminates_shell_key_step` | Pass |
| Wait key matches direct authorized wait submission | `shell::event_loop::tests::wait_key_matches_direct_wait_submission` | Pass |
| Continue key matches direct runtime advance | `shell::event_loop::tests::continue_key_matches_direct_runtime_advance` | Pass |
| Enter on selected Actions-pane row matches direct semantic-action submission | `shell::event_loop::tests::enter_on_selected_action_matches_direct_submission` | Pass |
| Launch option reaches fullscreen shell while default launch stays line-loop | `launch::tests::resolve_fullscreen_fixture_and_actor_binds_named_actor` and `usage_mentions_fullscreen_without_changing_default` | Pass |
| Reducer selected-action activation uses authorized submission | `intent::reducer::tests::activate_selection_submits_selected_action_through_authorized_path` | Pass |
| Existing intent/reducer contract remains no-direct-dispatch | `cargo test -p tracewake-tui --test intent_conformance` and full workspace gate | Pass |

## Anti-Contamination Results

- The shell stores only `PresentationState` and `TerminalSize`; it stores no
  simulation copy, event log, physical-state handle, or actor knowledge.
- Key input maps to the existing `UiIntent` stream. World effects are routed
  through `intent::reducer::reduce`, `TuiApp::submit_semantic_action`, and
  `TuiApp::advance_until`; no direct `RuntimeCommand` or event-application path
  was added in the shell.
- Redraw derives from `TuiApp::current_view()` and the Spec 0064
  `build_embodied_screen_model` / `render_embodied_to_buffer` path, preserving
  actor-filtered embodied rendering.
- The line loop remains a non-interactive test/automation driver. The default
  launch path is unchanged; fullscreen is explicit via `--fullscreen`.
- No acceptance step requires a TTY. The live terminal path is covered through
  injected lifecycle, key mapping, shell-step, and buffer-output witnesses.

## Verdict

Spec 0065 is accepted for the scoped fullscreen crossterm shell feature line
through implementation commits `43e06fa`, `cd6f162`, and `1df5719`, plus the
evidence/report commit that adds this artifact and guard truthing. The evidence
proves terminal lifecycle restore, script-equivalent key intents, actor-filtered
redraw through the Spec 0064 buffer seam, shell-path parity for wait/continue
and selected action submission, explicit fullscreen launch wiring, and retained
line-loop automation.

This report does not certify latest main after the archive/truthing commit,
Phase-4 entry, second-proof entry, institutions, notices, travel, LOD,
LLM/speech, story-sifting, `P0-CERT`, `FIRST-PROOF-CERT`, or any whole-project
status. It mints no invariant, gate, glossary term, or risk ID.
