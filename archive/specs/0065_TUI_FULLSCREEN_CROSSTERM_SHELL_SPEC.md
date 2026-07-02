# 0065 TUI Fullscreen Crossterm Shell Spec

**Status**: COMPLETED

This is a post-`FIRST-PROOF-CERT` **feature/capability** spec in the parallel `specs/NNNN`
series. It is staged in `specs/` and promoted to `archive/specs/` on acceptance; it is never
promoted to the live `docs/4-specs/` tier. It is not a certification audit, mutation-remediation
pass, Phase-4 entry claim, or latest-`main` certification. It uses the sibling hardening-spec
house structure (`0046`, `0047`, `0057`).

This is **Spec E** of the TUI Experience Overhaul roadmap
(`reports/tui-experience-overhaul-research-report.md` §3.7, §5.4 Stage D and Stage E, §7). It adds the
real terminal event loop with `crossterm` (raw mode, alternate screen) that reads key events (Stage D),
translates them into the Spec `0062` `UiIntent` stream, and redraws the Spec `0064` panes from the
current view model, and retires/reframes the old line loop (Stage E). The shell is strictly an
input/output **adapter**; it holds only presentation state.

## 0. Baseline statement and source discipline

- **Driver.** The research report's requirement that the fullscreen shell surround, not replace, the
  pure seam. Current entrypoint is a line loop: `run_command_loop`
  (`crates/tracewake-tui/src/run.rs`) prints `render_current_view`, reads a line, dispatches, reprints.
- **Baseline commit.** Re-verified against repository `HEAD`
  `14fda06c24515bd098826a77b1d4d6357cb59dd7` (post-`0064` landing; the Spec `0061` `EmbodiedScreenModel`
  and Spec `0064` `ratatui` buffer surfaces this spec builds on exist at this commit). Named symbols
  authoritative; line numbers provenance only.
- **Source discipline.** Commit hashes are audit/provenance only. No latest-`main` certification,
  Phase-4 entry, or second-proof entry. A manifest is path inventory only.
- **Authority posture.** Subordinate to foundation → architecture → execution → reference. Routes
  doctrine sharpening as *substance + home* (§5); ratifies no wording; mints no id.
- **Execution admissibility posture.** `P0-CERT not applicable`.
- **Ledger timing.** Archived-row entry at acceptance/closeout, not at proposal. No ledger row now.

## 1. Scope

### 1.1 In scope

In `tracewake-tui` only:

1. **`crossterm` dependency + terminal lifecycle:** enter/exit alternate screen and raw mode with
   guaranteed restore on exit/panic; handle resize.
2. **Event loop:** poll `crossterm` key/resize events, translate each key event into a `UiIntent` value
   through a new shell-owned `KeyEvent`→`UiIntent` mapping (the Spec `0062` `parse_key_token` maps
   *script tokens*, not live `crossterm` `KeyEvent`s), then feed that intent to the Spec `0062` `reduce`
   seam, update presentation state or submit through authorized core methods, and redraw the Spec `0064`
   `ratatui` buffer from the current `EmbodiedViewModel`. The live mapping must yield the same intent
   stream as the script token→intent mapping for equivalent keys (§4.3, §6); sharing the token→intent
   core so equivalence holds by construction, not only by test, is preferred.
3. **Presentation-only shell state:** focused pane, selected row, scroll offsets, help-overlay state,
   last terminal size. The shell keeps **no** simulation copy and reuses view models as the single
   source.
4. **Retire/reframe the line loop:** the fullscreen shell becomes the interactive entrypoint; the
   automation need is preserved by the Spec `0062` first-class headless/script runner, not by an
   undocumented REPL alias.

### 1.2 Out of scope (non-goals)

- No `UiIntent` definition or reducer (Spec `0062`); this spec *feeds* it with real key events.
- No new panes/layout (Spec `0064`) and no core view-model change.
- No default-launch flip decision beyond gating: fullscreen becomes default only after the headless
  dump + intent driver are green and CI-stable (report §9.1 #5) — the flip itself is confirmed in Spec
  `0069` acceptance.
- No debug dashboard (Spec `0068`) or conversation panel (Spec `0067`).

## 2. Dependencies and ordering

- **Depends on:** `0061` (screen model), `0062` (intent reducer), `0064` (panes/buffer) — all three are
  landed and archived (`docs/4-specs/SPEC_LEDGER.md`, status Completed). `0064` is a hard dependency at
  the redraw seam: §1.1 item 2 redraws the Spec `0064` `ratatui` buffer.
- **Blocks:** `0068` (debug dashboard runs inside the shell), and contributes to Spec `0069`.

## 3. Doctrine anchors

- **Foundation 08**: the TUI is the first real client; embodied play is the default surface.
- **Architecture 10**: world-advancing controls route through core/runtime; the client holds no live
  physical-state handle.
- **Architecture 04 / Execution 05**: no direct dispatch; the shell submits through the pipeline only.

## 4. Findings and remediation requirements

- **4.1 Shell surrounds the seam (seam: event loop).** The interactive loop is an adapter around
  `EmbodiedViewModel → EmbodiedScreenModel → buffer`; it never sits between core and the view model.
- **4.2 Guaranteed terminal restore (seam: lifecycle).** Raw mode/alternate screen are restored on
  normal exit and on panic; no terminal is left corrupted.
- **4.3 Same intents as scripts (seam: key→intent translation).** Real key events produce the identical
  `UiIntent` stream the Spec `0062` scripts do, so interactive and headless play cannot fork semantics.
- **4.4 No TTY required for CI (seam: test strategy).** Headless tests (Specs `0061`/`0062`) remain
  authoritative; interactive smoke tests are best-effort where feasible.
- **4.5 Implementation decomposition.** Terminal lifecycle + restore guard; event loop + key→intent
  translation; shell presentation-state struct; entrypoint switch + line-loop reframing. Separate
  tickets.

## 5. Doctrine amendments (routed as substance + home; not ratified here)

- **Architecture 10** — recognize the interactive shell as an adapter that holds only presentation
  state around the pure client seam (shared with Spec `0070`). Substance only.

## 6. Required fixtures and tests

- Headless equivalence: a key script (Spec `0062`) and the shell's key→intent translation produce the
  same intent stream for the same keys (unit-level translation test; no TTY).
- Lifecycle: raw-mode/alternate-screen enter and guaranteed restore verified in a controlled test.
- Regression: driving wait/continue/action through the shell path yields the same view-model outcomes
  as the line/script path (no semantic fork).

## 7. Acceptance artifact and evidence

A review artifact recording key→intent equivalence, the restore-on-exit/panic guarantee, and the
no-semantic-fork regression, at the exact implementation commit. Headless tests remain the
authoritative acceptance lane.

## 8. Implementation constraints

- TUI-only shell adapter; `crossterm` is the only new dependency here.
- Shell holds only presentation state; no simulation copy, no physical-state handle.
- No direct event application; all world change goes through authorized core methods.

## 9. Risks and open questions

- **Risk: raw-mode fullscreen breaks stdin/stdout drivers.** Mitigation: the headless script runner is
  first-class and CI-authoritative; the shell is additive.
- **Open question (§9.1 #5):** when fullscreen becomes default — only after headless dump + intent
  driver are green and CI-stable.

## 10. Invariants alignment

| Invariant | Stance | Rationale (mechanism @ surface) |
|---|---|---|
| INV-069 (TUI must not implement simulation rules) | aligns | Shell is an I/O adapter; no rule or truth read. |
| INV-104 (no direct primitive dispatch) | aligns | Shell submits through the pipeline via authorized methods. |
| INV-108 (possession is cognition-neutral) | aligns | Interactive input maps to the same intents/authority as scripts. |
| INV-065 (TUI is a primary product interface) | aligns | Delivers the interactive fullscreen client. |
| INV-018 (deterministic replay is foundational) | aligns | Shell state is presentation-only; world remains event-sourced/replayable. |

## Outcome

Completed: 2026-07-02

Implemented by the `0065TUIFULCRO` ticket series and accepted with
`archive/reports/0065_tui_fullscreen_crossterm_shell_acceptance.md`.

What changed:

1. `0065TUIFULCRO-001` added the `crossterm` dependency, a `shell` module, and
   a terminal lifecycle guard that restores raw mode and alternate screen on
   normal drop and panic unwind.
2. `0065TUIFULCRO-002` added live `KeyEvent` to `UiIntent` mapping, shell-local
   presentation/terminal-size state, the fullscreen event-loop seam, and redraw
   through `TuiApp::current_view()` into the Spec 0064 buffer renderer.
3. `0065TUIFULCRO-003` wired `--fullscreen [fixture_id] [actor_id]` as the
   explicit interactive shell entrypoint while preserving the existing line loop
   as the default automation path; it also made `ActivateSelection` submit the
   selected Actions-pane semantic action through the authorized reducer path.
4. `0065TUIFULCRO-004` added the acceptance artifact and truthed the workspace
   source/dependency guard for the new TUI shell files and `crossterm`.

Verification:

1. `cargo fmt --all --check` — passed after the acceptance report and ticket
   outcome were added.
2. `cargo clippy --workspace --all-targets -- -D warnings` — passed after the
   acceptance report and ticket outcome were added.
3. `cargo build --workspace --all-targets --locked` — passed after the
   acceptance report and ticket outcome were added.
4. `cargo test --workspace` — passed after the acceptance report and ticket
   outcome were added.

Deviations:

1. The default launch flip remains deferred to Spec 0069 as specified; this
   series adds an explicit `--fullscreen` launch option.
2. No live TTY smoke was required or run. The accepted evidence is the
   headless/no-TTY lane: injected terminal lifecycle tests, key mapping
   equivalence, shell-step parity, and buffer-output witnesses.
3. The report/evidence commit updated the anti-regression guard allowlists for
   `crates/tracewake-tui/src/shell/*.rs` and `crossterm` after the first
   workspace test exposed the missing classifications.

Commit roles:

1. Implementation baseline commits: `43e06fa`, `cd6f162`, and `1df5719`.
2. Evidence/report commit: `22d7cf9`, which added the acceptance report and
   archived `0065TUIFULCRO-004`.
3. Archive/truthing commit: the commit that archives this spec/report and
   updates the ledger.

This closeout does not certify latest main after the archive/truthing commit,
Phase-4 entry, second-proof entry, institutions, notices, travel, LOD,
LLM/speech, story-sifting, `P0-CERT`, `FIRST-PROOF-CERT`, or any whole-project
status. It mints no invariant, gate, glossary term, or risk ID.
