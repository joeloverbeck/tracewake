# 0024PHA3ACONSCH-009: TUI operator-command quarantine, debug-command gating, and render-guard widening

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-tui` (`input.rs`, `run.rs`, `app.rs`), TUI tests, `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`, plus help/README surfaces as surfaced.
**Deps**: None

## Problem

Spec 0024 findings `ORD-HARD-152`, `153`, `157` (all medium):

- `152`: `run no-human-day` is parsed as a top-level `UiCommand::OperatorProof`
  command — sibling of `view`/`wait`, ungated — that autonomously fast-forwards the
  whole world (the possessed actor included: `actor_ids: Vec::new()` fills from all
  actors via `config.actor_ids.is_empty()` in `scheduler.rs`) for a full day with no
  interruption stop and no missed-event summary, printing a `DEBUG NON-DIEGETIC`
  panel from the normal-play surface. The foundation doc stages interruption-stop /
  missed-event summaries, but no execution doc records the staging decision.
- `153`: every `render_debug` arm except `Overlay` dispatches without a
  `debug_available` check — the 0022-derived gate is decorative for the other
  panels (`debug beliefs <actor>` dumps any actor's beliefs with no binding).
- `157`: the render-reachability guard scans `include_str!("render.rs")` only;
  eleven `pub fn render_*_panel` functions in `debug_panels.rs` sit outside it —
  the exact `ORD-HARD-125` orphaned-renderer failure mode, unguarded for the panel
  file.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `4d62f61`: `parse_command` →
   `OperatorProofCommand::RunNoHumanDay` (`input.rs`), ungated dispatch in
   `run.rs::dispatch_command`; `render_debug`'s per-arm direct calls with only
   `DebugCommand::Overlay` routed through the gated
   `render_debug_embodied_overlay`; `render_reachability_errors` sourcing
   `include_str!("render.rs")` (`app.rs`); eleven `pub fn render_*_panel` in
   `debug_panels.rs` (count re-verified at spec reassessment).
2. Verified against spec 0024 §4 (`ORD-HARD-152`/`153`/`157`),
   `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` (time controls
   staged; "debug may not provide normal-play world-affecting shortcuts"), and
   `docs/2-execution/07_*` (no staging record — swept during the audit).
3. Cross-artifact boundary: the embodied-vs-debug command-surface contract (which
   verbs are normal play, which are operator/debug, and what gates the latter) —
   recorded in `docs/2-execution/07` by this ticket so the lineage's
   unrecorded-deferral rule is satisfied.
4. INV-068/INV-107 restated: debug is visibly non-diegetic and quarantined — debug
   omniscience must be structurally unable to feed ordinary cognition, and debug
   surfaces must not be normal-play world-affecting shortcuts.
5. Enforcement surface (debug quarantine): gating strengthens the boundary; the
   panels remain read-only `&self` renders (verified — no mutation/observation
   creation), so no cognition path changes. Embodied playability is untouched: the
   five-action embodied sweep does not use operator or debug commands. Existing
   debug-mode tests bind an actor first, so they remain green under gating.
6. Implementer-recorded choices (spec-assigned, recommendations adopted unless
   implementation falsifies them): (a) **classification** — `run no-human-day`
   becomes an operator/debug-prefixed command gated on derived `debug_available`;
   the player-facing time-acceleration feature (interruption stops, missed-event
   summaries) is recorded in `docs/2-execution/07` as staged Phase 3B+ work; (b)
   **gating policy** — all `DebugCommand` arms route through one gating helper
   consulting the same derived availability the overlay uses. Both decisions are
   recorded in the execution doc by this ticket.
7. Adjacent required consequences: moving the command updates its help text and the
   README sample session (the readme/help conformance tests currently bless the
   top-level form) — these surfaces join this ticket's scope as surfaced.

## Architecture Check

1. One gating helper consulted by every debug arm (instead of per-arm checks or
   gating only "sensitive" panels) makes the gate's coverage greppable and lockable
   — a guard can assert every `DebugCommand` match arm routes through it. Moving the
   world-advancing operator command behind the same gate resolves the
   classification honestly (it already prints a non-diegetic panel) without
   building the staged player feature prematurely. Widening
   `render_reachability_errors` to `debug_panels.rs` closes the single-file
   perimeter (the R-28 type-convention census shape) at its second instance.
2. No backwards-compatibility aliasing/shims: the top-level `run no-human-day`
   spelling is moved, not aliased; no ungated legacy arm survives.

## Verification Layers

1. INV-068 command-surface quarantine → command-loop test: `run no-human-day`
   (debug-prefixed form) is refused without debug availability; the old top-level
   spelling is an unknown command (test).
2. INV-107 gating coverage → guard asserting every `DebugCommand` arm routes
   through the gating helper; command-loop test issuing `debug beliefs actor_x`
   before any `bind` is refused (codebase grep-proof + test).
3. R-28 perimeter closure (`157`) → widened guard covers `debug_panels.rs`'s eleven
   panels; a synthetic uncalled `*_panel` fails (test).
4. Staging record → grep-proof: `docs/2-execution/07` carries the operator-proof
   classification, the gating policy, and the Phase 3B+ time-acceleration staging
   entry (manual review + grep).
5. Whole-pipeline → full workspace gates (transcript/readme conformance tests
   updated, still deterministic).

## What to Change

### 1. Operator-command quarantine (`152`)

Move `RunNoHumanDay` under the `debug ` prefix (operator-proof form); gate on
derived `debug_available`; update help text + README sample session + their
conformance tests.

### 2. Debug gating helper (`153`)

All `render_debug` arms route through one helper returning a typed
"debug unavailable" refusal absent availability; add the no-bind refusal test and
the all-arms-routed guard.

### 3. Render-guard widening (`157`)

`render_reachability_errors` gains `debug_panels.rs` as a producer source (eleven
panels enumerated); synthetic uncalled-panel negative.

### 4. Execution-doc record

`docs/2-execution/07` records both implementer decisions and the staging status of
player-facing time acceleration (interruption stop, missed-event summaries —
Phase 3B+).

## Files to Touch

- `crates/tracewake-tui/src/input.rs` (modify)
- `crates/tracewake-tui/src/run.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-tui/tests/command_loop_session.rs` (modify)
- `crates/tracewake-tui/tests/readme_sample_session.rs` (modify — as surfaced by the
  command move)
- `README.md` (modify — as surfaced: sample-session text naming the command)
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` (modify)

## Out of Scope

- Building player-facing time acceleration with interruption stops and missed-event
  summaries (recorded as staged Phase 3B+ — spec Out of scope).
- The debug overlay's own gating (landed in 0023; this ticket generalizes the
  pattern, it does not rework the overlay).
- Scheduler changes: the empty-`actor_ids` fill semantics are untouched; only the
  command surface that reaches it is reclassified.

## Acceptance Criteria

### Tests That Must Pass

1. The debug-prefixed run-no-human-day command is refused without debug
   availability and works with it; the old top-level spelling no longer parses; the
   world-advance behavior under the gated form is byte-identical to before
   (transcript test).
2. `debug beliefs <actor>` (and every other debug command) is refused with a typed
   message when no controller is bound; the all-arms-routed guard fails on a
   synthetic ungated arm.
3. The widened reachability guard covers all eleven `debug_panels.rs` panels and
   fails on the synthetic uncalled panel.
4. `docs/2-execution/07` grep-proofs: classification, gating policy, and staging
   entries present. The four workspace gates pass.

### Invariants

1. No world-advancing operator command is reachable from the embodied verb set
   outside the gated prefix.
2. Every debug surface is gated by the same derived availability, and the gate's
   coverage is structurally asserted.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/command_loop_session.rs` — no-bind refusal tests
   (debug command + operator command), gated-form world-advance transcript.
2. `crates/tracewake-tui/src/app.rs` (inline tests) — widened reachability guard +
   uncalled-panel synthetic; all-arms-routed gating guard.

### Commands

1. `cargo test -p tracewake-tui`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

Moved no-human day advancement out of the embodied command surface. The old
top-level `run no-human-day` spelling now returns an unknown-command error, while
the operator-proof form is `debug run no-human-day` and runs only after the shared
debug availability gate passes. Every `DebugCommand` arm now routes through the same
gate before dispatch, including debug panels and the world-advancing operator proof.

Widened the render reachability guard to include `debug_panels.rs`, added a
synthetic uncalled debug panel, and added a source guard proving `render_debug`
checks `debug_available` before matching any debug command. Updated command-loop
tests, TUI acceptance/adversarial scripts, README/help command docs, and
`docs/2-execution/07` with the operator-proof classification, gating policy, and
Phase 3B+ staging record for player-facing time acceleration.

Verification:

1. `cargo test -p tracewake-tui` passed.
2. `cargo fmt --all --check` passed.
3. `cargo clippy --workspace --all-targets -- -D warnings` passed.
4. `cargo build --workspace --all-targets --locked` passed.
5. `cargo test --workspace` passed.
