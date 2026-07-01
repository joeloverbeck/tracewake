# 0063CORACTKNO-005: TUI render disposition and anti-leak / parity / two-hop test suite

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — renders/disposes observed activity in `render.rs` and the screen dumps; adds the anti-leak / occlusion / stale / possession-parity / two-hop test suite
**Deps**: 0063CORACTKNO-003, 0063CORACTKNO-004

## Problem

The enriched `VisibleActor` (ticket 003) must be rendered with an explicit disposition for every
closed activity kind and source — no wildcard/default renderer path — and carried into the 0061
screen dumps, then proven by the full behavior-witness suite (Spec 0063 §4.4, §6, §7). Today the
actors loop prints identity only (`crates/tracewake-tui/src/render.rs:129` —
`format!("- {}", actor.actor_id.as_str())`), the text dump does the same
(`screen/text_dump.rs:102`), and the field-disposition guard
(`crates/tracewake-tui/tests/tui_seam_conformance.rs:190`) maps `local_actors` to `ActorsPane`
without activity. This ticket lands Hop 2 (`VisibleActor.observed_activity` → rendered/dumped pane)
and the negative evidence that the firewall holds: an activity present in truth but not perceived
produces no row; a mutant removing either hop fails (§6).

## Assumption Reassessment (2026-07-01)

1. Render seam: `render.rs:128` `for actor in &view.local_actors { … actor.actor_id … }`. Dump
   seams: `screen/text_dump.rs:100-102` (text `local_actors`), `screen/struct_dump.rs`, and
   `screen/model.rs` (`EmbodiedScreenModel.local_actors: Vec<VisibleActor>`, line 132; disposition
   guarded by `tui_seam_conformance.rs:190` `embodied_screen_model_field_disposition`, whose map
   includes `("local_actors", "ActorsPane", "view.local_actors.clone()")` at line ~222). The Hop-1
   test home is `crates/tracewake-core/tests/hidden_truth_gates.rs` — the existing two-hop test
   `actor_known_local_actor_reaches_embodied_view_model_with_context_provenance` (line 1749) already
   proves identity reaches `view.local_actors` and a hidden actor does not, via the local
   `embodied_view(&context, &world)` helper (line 127).
2. Spec 0063 §4.4 requires explicit per-kind/per-source disposition (no wildcard), enforced long-term
   by the Spec 0069 conformance guard; §6 enumerates the required tests (positive; anti-leak omission;
   occlusion; stale with mandatory wording; possession parity; two-hop with a hop-removal mutant);
   §7 requires each guard to carry a behavior witness (report §6.5 non-vacuity).
3. **Shared boundary under audit:** `VisibleActor` (ticket 003) → `render.rs` / screen dumps, plus
   the field-disposition conformance guard. The dump/screen-model surfaces are the same three tui
   files ticket 003 touched for compile (`text_dump.rs`, `struct_dump.rs`, `model.rs`); this ticket
   is the sequential follow-on that adds real disposition (Deps 003 ⇒ no merge conflict). Possession
   parity and occlusion/stale assertions consume ticket 004's fixtures.
4. **Motivating invariants.** INV-093 (actor-knowledge leakage is a high-severity defect, tested):
   the anti-leak/occlusion tests are the mandated leakage tests. INV-008/069 (UI is not authority;
   TUI implements no rules): render disposes pre-resolved view-model fields, deriving nothing.
   INV-024/067 (no telepathy; embodied shows actor-known reality): omission is correct when the
   actor did not perceive the activity. INV-094 (possession parity is tested).
5. **Enforcement surface — actor-knowledge firewall + no TUI derivation.** The render/dump code must
   read only `VisibleActor` fields (no join against `actor_id`, action registry, routine, scheduler,
   or physical state — §1.2, §4.5). The negative tests are the enforcement: (a) truth-has-activity /
   actor-unperceived → no activity row (anti-leak); (b) occluded → no row; (c) stale → mandatory
   stale wording, and removing the stale label fails a test; (d) possession parity — same fixture
   under two possessions yields each actor's own activity view; (e) two-hop — a mutant removing Hop 1
   (core → `observed_activity`) or Hop 2 (`observed_activity` → dump) fails. No determinism change:
   rows stay deterministically ordered (ticket 003).
6. **Schema extension (additive).** Extends the 0061 `EmbodiedScreenModel` / `ScreenDump` to carry
   the activity fields into text/struct dumps (§2 dependency on 0061). Consumers: `screen/model.rs`
   (`build_embodied_screen_model` + the disposition map), `screen/text_dump.rs`, `screen/struct_dump.rs`,
   and the `tui_seam_conformance.rs` field-disposition guard (a new activity disposition entry).
   Extension is **additive**: existing panes/dumps keep their fields; the activity is a new pane
   element / dump line. No back-compat shim.

## Architecture Check

1. Disposing each closed activity kind/source with an explicit arm — and making the field-disposition
   conformance guard require it — is cleaner and safer than a `Display`-style fallback: it makes an
   unhandled variant a compile/guard failure rather than a silent debug-string leak (§4.4, §4.5). The
   negative tests co-locate in `hidden_truth_gates.rs` with the existing firewall witnesses, reusing
   the `embodied_view` helper so the new proofs share the established seam.
2. No backwards-compatibility aliasing/shims: render/dump gain new explicit arms; no fallback path is
   introduced. The stale/uncertain wording is required, not optional, so there is no "clean" default
   that could mask staleness.

## Verification Layers

1. INV-093 / INV-024 (leakage tested; no telepathy) -> replay/golden-fixture check + manual review:
   the anti-leak and occlusion tests (over ticket-004 fixtures) assert no activity row when the actor
   did not perceive it; a hop-removal mutant fails.
2. INV-008 / INV-069 (no TUI authority) -> codebase grep-proof: `render.rs`/dumps reference only
   `VisibleActor` fields; the `render_embodied_view_uses_sealed_view_model_accessors` conformance
   test (`tui_seam_conformance.rs:166`) stays green and the field-disposition map covers activity.
3. INV-094 (possession parity) -> replay/golden-fixture check: the parity test runs one fixture under
   two possessions and asserts each actor's own activity view, neither seeing the other's hidden state.
4. INV-112 (holder-known freshness) -> manual review + test: the stale test asserts mandatory stale
   wording; removing the label fails.

## What to Change

### 1. Render disposition

In `crates/tracewake-tui/src/render.rs`, replace the identity-only actors loop with an explicit
per-kind/per-source disposition of `VisibleActor.display_label`, presence (source + freshness +
uncertainty), and `observed_activity` — one arm per `ObservedActorActivityKind` and
`ActorKnownActivitySourceKind`, no wildcard.

### 2. Screen-dump disposition

In `crates/tracewake-tui/src/screen/{text_dump,struct_dump,model}.rs`, carry the activity into the
text and structured dumps and the `EmbodiedScreenModel`, and add the activity disposition entry to
`tui_seam_conformance.rs`'s `embodied_screen_model_field_disposition` map so the guard enforces it.

### 3. Behavior-witness test suite

In `crates/tracewake-core/tests/hidden_truth_gates.rs`, extend the two-hop test and add: positive
(visible activity → row with source + freshness), anti-leak (truth activity, unperceived → no row),
occlusion (no cue → no row), stale (mandatory wording; label-removal fails), possession parity (one
ticket-004 fixture under two possessions). Add the Hop-2 render/dump assertions in
`crates/tracewake-tui/tests/tui_seam_conformance.rs` (or the render test module), including a
hop-removal negative.

## Files to Touch

- `crates/tracewake-tui/src/render.rs` (modify)
- `crates/tracewake-tui/src/screen/text_dump.rs` (modify)
- `crates/tracewake-tui/src/screen/struct_dump.rs` (modify)
- `crates/tracewake-tui/src/screen/model.rs` (modify)
- `crates/tracewake-tui/tests/tui_seam_conformance.rs` (modify)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify)

## Out of Scope

- Producing/sealing activity (ticket 002) and the `VisibleActor` shape/transfer (ticket 003).
- Authoring the fixtures themselves (ticket 004) — this ticket asserts over them.
- The consolidated acceptance artifact (ticket 006).
- Any `ratatui`/`crossterm` fullscreen shell (Spec 0065) or pane layout (Spec 0064).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-tui` — render + dump dispose every activity kind/source; the
   field-disposition guard covers activity; identity-only dumps remain byte-stable.
2. `cargo test -p tracewake-core --test hidden_truth_gates` — positive, anti-leak, occlusion, stale
   (with label-removal failure), and possession-parity witnesses pass; the two-hop test asserts
   Hop 1 and a hop-removal mutant would fail.
3. `cargo test --workspace` — full suite green (end-to-end pipeline proof).

### Invariants

1. Render/dump derive nothing from `actor_id`/registry/routine/scheduler/physical state; they read
   only `VisibleActor` fields, and every closed variant has an explicit disposition (INV-008/069,
   INV-093, §4.4/§4.5).
2. Omission is correct when unperceived/occluded; stale carries mandatory wording; each possession
   sees only its own actor-known activity (INV-024/067, INV-094, INV-112).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs` — positive/anti-leak/occlusion/stale/parity/
   two-hop witnesses (each a behavior witness per §6.5, not a shape-only check).
2. `crates/tracewake-tui/tests/tui_seam_conformance.rs` — activity field-disposition entry + Hop-2
   render/dump assertions + hop-removal negative.

### Commands

1. `cargo test -p tracewake-core --test hidden_truth_gates && cargo test -p tracewake-tui`
2. `cargo test --workspace`
3. The two-crate + workspace boundary is correct because the firewall witnesses live in core while
   the render/dump disposition lives in tui; the capstone (006) consolidates the evidence artifact.

## Outcome

Completed: 2026-07-01

Implemented actor activity disposition for the ordinary TUI render and screen dump actor pane via
a shared `VisibleActor` formatter. The formatter has explicit arms for every
`ObservedActorActivityKind` and `ActorKnownActivitySourceKind`, preserves legacy identity-only test
rows, and renders actor-safe source kind/tick/staleness/uncertainty without exposing raw event ids
or source-summary internals in ordinary UI output. Updated the TUI conformance guard to require the
actor formatter and to fail when either closed taxonomy gains an undisposed variant. Updated parity
goldens for real local actor rows that now include presence/freshness and `activity=not apparent`.

Added core hidden-truth witnesses for Hop 1: positive observed activity transfer, unperceived
activity omission, occluded actor omission, stale activity label/uncertainty preservation, and
possession parity. Added TUI Hop 2 render/dump tests for rich activity rows, identity-only row
stability, closed taxonomy disposition, and raw event-id non-leakage. Added the ticket-004 fixture
files to the workspace source-classification census after the workspace gate identified the new
content files.

Implementation note: `tracewake-core` has no dependency or dev-dependency on `tracewake-content`, so
the core firewall witnesses intentionally use sealed `KnowledgeContext` inputs at the existing
`hidden_truth_gates` seam instead of importing the ticket-004 fixtures directly. The ticket-004
fixtures remain content substrate and are covered by the content/golden corpus; ticket 006 will
consolidate this evidence.

Verification:

1. `cargo fmt --all --check`
2. `cargo test -p tracewake-core --test hidden_truth_gates`
3. `cargo test -p tracewake-tui`
4. `cargo test --workspace`
