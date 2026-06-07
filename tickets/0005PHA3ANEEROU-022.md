# 0005PHA3ANEEROU-022: Embodied view-model — needs/intention/routine summary, affordances, why-not

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — extends the embodied view model and TUI to expose Phase 3A needs/intention/routine status, sleep/eat/work/wait/continue affordances, and why-not, without hidden-truth leakage.
**Deps**: 0005PHA3ANEEROU-011, 0005PHA3ANEEROU-012

## Problem

The possessed actor's embodied view must expose Phase 3A mechanics in actor-known terms — a banded need/status summary, the current intention/routine summary, available sleep/eat/work/wait/continue actions, and why-not for unavailable ones — while stopping on actor-perceived salient interruptions and leaking no hidden true food location, true routine-failure cause, or other-actor debug state (Spec 0005 §14.1, §14.3; `INV-024`). This is the playable surface that makes the possessed actor exercise the same ordinary actions as autonomous actors.

## Assumption Reassessment (2026-06-07)

1. The embodied view model is generated in `crates/tracewake-core/src/view_models.rs` and rendered/handled through the TUI (`crates/tracewake-tui/src/app.rs`, `render.rs`, `input.rs`); Phase 2A already exposes embodied views, semantic action menus, wait, why-not, and notebooks (Spec §4.1). This ticket extends that surface, not a parallel one.
2. The cognition state (needs 001, intention/candidate 002/012, routine 003) and the ordinary actions (007–011) provide the data and affordances. Spec §14.1 fixes the embodied surface (need/status summary, intention/routine summary in actor-known terms, sleep/eat/work/wait/continue affordances, why-not, wait/continue time control, stop-on-salient-interruption, no hidden-truth leakage). Spec §14.1 also fixes that need values are banded/labeled ("hungry"/"very hungry"), exact numbers debug-only.
3. Shared boundary under audit: the embodied view is the actor-knowledge filter surface — it must render only actor-known facts (banded needs, believed routine state), never other actors' needs/intentions, the true food location, or the true routine-failure cause. `view_models.rs` is the canonical filter; the TUI only renders it (`INV-008` — the view decides nothing).
4. Invariant motivating this ticket: `INV-024` — "No telepathy" (the embodied view must not reveal hidden information the actor-knowledge filter forbids). The need summary is banded; the intention/routine summary is in actor-known terms; why-not explains in actor-known terms ("cannot eat: no accessible known food") without revealing the true cause.
5. Actor-knowledge / no-leak surface: this IS an actor-knowledge filter surface. The view model exposes banded needs + believed routine/intention summary + affordances filtered to the possessed actor; it must not include another actor's cognition or ground truth. This ticket adds no leakage path — it reuses the Phase 2A embodied filter discipline and keeps exact values/other-actor state debug-only (ticket 023). The no-leak proof is the capstone (025, cited).

## Architecture Check

1. Generating the Phase 3A embodied surface in `view_models.rs` (the existing actor-filtered projection) and rendering it in the TUI keeps the no-leak filter in one authoritative place with the view as pure presentation (`INV-024`, `INV-008`) — putting need/routine logic in the TUI would risk a leak path and split the filter. Banded needs + actor-known summaries by default keep mechanics legible without exposing raw truth.
2. No backwards-compatibility shims: extends the existing embodied view-model/TUI surface; affordances reuse the semantic action menu rather than new player-only commands (Spec §3.4).

## Verification Layers

1. No telepathy (`INV-024`) -> manual review + view-model test: the possessed actor's embodied view contains only its own banded needs and believed routine/intention summary; a test proves it omits other actors' cognition and the true food location/failure cause.
2. Possession parity / ordinary actions (Spec §3.4) -> view-model test: sleep/eat/work/wait/continue appear in the semantic action menu when valid for the possessed actor; unavailable ones carry an actor-known why-not.
3. Presentation-only (`INV-008`) -> codebase grep-proof: the TUI renders the view model and submits proposals through the pipeline; it computes no need/affordance legality itself.

## What to Change

### 1. Embodied view-model extension

Extend `crates/tracewake-core/src/view_models.rs` so the possessed actor's embodied view exposes: a banded need/status summary, the current intention/routine summary in actor-known terms, the available sleep/eat/work/wait/continue affordances (with why-not for unavailable ones), and salient-interruption stop signals. All fields are actor-known-filtered; exact numeric needs are not included by default.

### 2. TUI rendering + input

Extend the TUI (`crates/tracewake-tui/src/render.rs`, `app.rs`, `input.rs`, `run.rs` as needed) to render the needs/intention/routine summary and surface the new affordances in the semantic action menu (by stable ID and numeric selection), with wait/continue time control stopping on possessed-actor-perceived salient interruptions.

## Files to Touch

- `crates/tracewake-core/src/view_models.rs` (modify — Phase 3A embodied fields, actor-known-filtered)
- `crates/tracewake-tui/src/render.rs` (modify — render needs/intention/routine summary)
- `crates/tracewake-tui/src/app.rs` (modify — Phase 3A affordances in the action menu, salient-interruption stop)
- `crates/tracewake-tui/src/input.rs` (modify — sleep/eat/work/wait/continue selection)
- `crates/tracewake-tui/src/run.rs` (modify — time-control loop interaction, if needed)

## Out of Scope

- Debug surfaces (full needs/routines/planner/stuck for all actors, ticket 023) — embodied is actor-known only.
- Core action/cognition logic (tickets 007–014).
- The no-leak acceptance proof over a transcript (ticket 025).

## Acceptance Criteria

### Tests That Must Pass

1. The possessed actor's embodied view shows banded needs and a current intention/routine summary in actor-known terms; sleep/eat/work/wait/continue appear when valid, with why-not for unavailable ones.
2. The embodied view omits other actors' needs/intentions, the true food location, and the true routine-failure cause (no-leak view-model test).
3. The TUI submits the new affordances through the shared pipeline and renders the view model without computing legality itself (grep-proof).

### Invariants

1. The embodied view reveals only actor-known information; no hidden-truth leakage (`INV-024`).
2. Ordinary actions are exposed through the shared semantic menu, not player-only commands; the view is presentation-only (`INV-008`, Spec §3.4).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/embodied_flow.rs` (modify) — needs/intention/routine summary, affordance availability + why-not, no-leak omission.
2. `crates/tracewake-core/src/view_models.rs` (unit tests) — actor-known filtering of Phase 3A fields.

### Commands

1. `cargo test -p tracewake-tui --test embodied_flow`
2. `cargo test -p tracewake-core view_models`
3. TUI + view-model scope is the correct embodied boundary; the full no-leak transcript proof is the capstone (025).
