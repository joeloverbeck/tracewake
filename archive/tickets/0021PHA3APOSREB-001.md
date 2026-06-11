# 0021PHA3APOSREB-001: Possession-rebind rejection hygiene, embodied provenance partition, and frontier marking

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-tui` (`app`, `render`), `tracewake-core` (`projections`, anti-regression guard); `docs/3-reference/01_DESIGN_RISK_REGISTER.md` Watch entry; conformance-index row
**Deps**: `specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md` (ORD-HARD-066, ORD-HARD-082, ORD-HARD-096; §6)

## Problem

`TuiApp::bind_actor` never clears `self.last_rejection` (while `run_no_human_day`
does), and `build_embodied_view_model` maps a rejection report into
`last_rejection_summary`/`last_rejection_why_not` without checking
`ValidationReport.actor_id` against the viewer — so after bind A → blocked action →
bind B, actor B's embodied view renders actor A's why-not facts: knowledge transfer
through possession (`ORD-HARD-066`, high; INV-006/067/093/108). Adjacent on the same
surface: `with_validator_availability` copies the pipeline's **unfiltered**
`report.checked_facts` into embodied `provenance_refs`, ignoring the
`is_actor_visible_fact` partition (`ORD-HARD-082`), and the embodied
"Knowledge context:" line prints the global event-log frontier unmarked as
non-diegetic (`ORD-HARD-096`). One ticket because all three are the possession/embodied
epistemic-hygiene surface the spec's §8 groups first.

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `89059a5`: `bind_actor` (`crates/tracewake-tui/src/app.rs`)
   attaches the controller, records perception, sets `bound_actor_id`, and does not
   touch `last_rejection`; `run_no_human_day` clears it; `current_view` passes
   `self.last_rejection.as_ref()` into `build_embodied_view_model`
   (`crates/tracewake-core/src/projections.rs`), which maps it unconditionally;
   `ValidationReport` carries `actor_id: Option<ActorId>`. `with_validator_availability`
   (projections.rs) extends `provenance_refs` from `report.checked_facts` while the
   pipeline partitions facts via `is_actor_visible_fact`
   (`crates/tracewake-core/src/actions/pipeline.rs`). `render_embodied_view`
   (`crates/tracewake-tui/src/render.rs`) prints
   `Knowledge context: id=… frontier=…` with no `DEBUG NON-DIEGETIC` marker; the
   frontier is the global `log.events().len()` stamped in `current_view_context`
   (app.rs).
2. Verified against spec 0021 (reassessed 2026-06-11; every finding operator-verified,
   zero refuted): findings ORD-HARD-066/082/096; §6 assigns the risk-register Watch
   entry to the first group; existing rebind gate
   (`crates/tracewake-tui/tests/adversarial_gates.rs::…possession_rebind_does_not_transfer…`)
   rebinds only after accepted actions, so `last_rejection` is `None` in it — the gap
   this ticket's new gate closes.
3. Cross-artifact boundary under audit: the `ValidationReport` flow from the action
   pipeline through the TUI host seam into the embodied view model — specifically the
   actor-visible vs. debug-only fact partition and the report-to-viewer binding.
4. INV-006 restated: possession transfers no world knowledge. INV-067: embodied mode
   shows the current actor's known world only. INV-093: actor-knowledge leakage into
   embodied view models is a high-severity defect. INV-108: possession is
   cognition-neutral — binding changes input and viewpoint only.
5. Actor-knowledge-filtering surface touched: all three changes strictly narrow what
   reaches the embodied view (clear-on-rebind, viewer-keyed report guard,
   actor-visible-partition sourcing); no hidden-truth path is added. No
   deterministic-replay surface is touched: `last_rejection` is host-side display
   state, never logged or checksummed; `provenance_refs` is view-model output, not
   canonical state.
6. View-model shape: no schema extension — `provenance_refs` and the rejection fields
   keep their types; only their population narrows, and the frontier line changes
   rendering/marking only. Additive-vs-breaking is N/A (no shape change).

## Architecture Check

1. The fix lands at both layers deliberately: the host seam clears `last_rejection` in
   `bind_actor` (the asymmetry with `run_no_human_day` proves the omission), AND
   `build_embodied_view_model` drops any report whose `actor_id` is not the viewer —
   defense in depth so a future host cannot reintroduce the leak. Provenance sourcing
   moves to the actor-visible partition, mirroring the render path's existing why-not
   choice (`actor_visible_facts` only) — one authority for what an actor may see,
   instead of two divergent consumers of the same report.
2. No backwards-compatibility aliasing/shims: the unfiltered `checked_facts` sourcing
   is replaced, not kept behind a flag; the unmarked frontier line is corrected, not
   duplicated.

## Verification Layers

1. INV-006/INV-108 (rebind transfers nothing) -> new TUI adversarial gate: bind A,
   submit a blocked action whose rejection carries actor-visible facts, rebind B,
   assert B's view model carries no rejection fields.
2. INV-067 (viewer-keyed reports) -> core unit test: a `ValidationReport` with a
   mismatched `actor_id` yields `None` rejection fields regardless of host behavior.
3. INV-067/INV-093 (partition honored) -> core test seeding a debug-only validator
   fact and asserting it is absent from embodied `provenance_refs`, plus a source
   guard asserting `projections.rs` production code never reads `.checked_facts`.
4. INV-068 hygiene (frontier marking) -> render test asserting the knowledge-context
   line is marked `DEBUG NON-DIEGETIC` (or the frontier is viewer-scoped — whichever
   the implementation picks, the test pins it).

## What to Change

### 1. Clear rejection state on rebind (ORD-HARD-066, host half)

`TuiApp::bind_actor` sets `self.last_rejection = None` alongside `bound_actor_id`.

### 2. Viewer-keyed rejection guard (ORD-HARD-066, core half)

`build_embodied_view_model` drops a rejection report whose `report.actor_id` does not
match `context.viewer_actor_id()` before mapping the rejection fields.

### 3. Provenance partition (ORD-HARD-082)

`with_validator_availability` sources embodied `provenance_refs` from the
actor-visible partition (`report.actor_visible_facts`), not `report.checked_facts`;
add the source guard banning `.checked_facts` reads in `projections.rs` production
code (anti-regression guard, source-scan style).

### 4. Frontier marking (ORD-HARD-096)

Mark the embodied "Knowledge context:" line `DEBUG NON-DIEGETIC`, or scope the
displayed frontier to the viewer's projection — pick one, pin it with the render
test, and record the choice in this ticket's closure note.

### 5. Documentation (§6, first-group share)

Add the **Guard Vacuity / Decorative Lock** Watch entry to
`docs/3-reference/01_DESIGN_RISK_REGISTER.md` (guardrail: every lock ships a synthetic
negative proving it fires on behavior; escalation trigger: a new guard whose only
assertions are presence/shape/count checks over a hand-maintained artifact). Add the
possession rebind-rejection gate conformance row to
`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`.

## Files to Touch

- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-tui/src/render.rs` (modify)
- `crates/tracewake-tui/tests/adversarial_gates.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — `.checked_facts` source guard)
- `docs/3-reference/01_DESIGN_RISK_REGISTER.md` (modify)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)

## Out of Scope

- The fabricating planning-context builders and hidden-truth gate rebuild
  (`ORD-HARD-068` — `archive/tickets/0021PHA3APOSREB-002.md`).
- The dead-surface sweep extension and `typed_leads`/`debug_available` surfacing
  (`ORD-HARD-083` — ticket 0021PHA3APOSREB-010).
- The INV-087 bind-time-perception decision (`ORD-HARD-095` — ticket
  0021PHA3APOSREB-012); this ticket does not move or suppress the bind-time
  perception events.
- Conformance rows for sibling tickets' locks (each lands with its enforcing ticket).

## Acceptance Criteria

### Tests That Must Pass

1. New rebind-after-rejection adversarial gate (bind A → blocked action → bind B →
   B's embodied view and rendered output carry no rejection summary, no why-not
   facts).
2. Core unit test: mismatched-`actor_id` report → `None` rejection fields; matching
   report → fields present (control).
3. Core test: a debug-only validator fact never appears in embodied
   `provenance_refs`; source guard rejects `.checked_facts` reads in projections
   production code (synthetic violation fails).
4. Render test pins the frontier-line marking/scoping choice.
5. `cargo test --workspace` green.

### Invariants

1. No rejection report crosses an actor boundary: embodied rejection fields render
   only for the actor the report was issued to.
2. Embodied `provenance_refs` ⊆ the pipeline's actor-visible fact partition — the
   debug-only partition is unreachable from the embodied view model.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/adversarial_gates.rs` — rebind-after-rejection gate
   (the existing rebind gate covers accepted actions only).
2. `crates/tracewake-core/src/projections.rs` (mod tests) — viewer-guard unit test +
   debug-only-fact partition test.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — `.checked_facts` source
   guard with synthetic violation.
4. `crates/tracewake-tui/src/render.rs` (mod tests) — frontier-line marking test.

### Commands

1. `cargo test -p tracewake-tui --test adversarial_gates`
2. `cargo test -p tracewake-core projections`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-11

What changed:

- `TuiApp::bind_actor` now clears `last_rejection`, so host-side why-not state cannot survive a possession rebind.
- `build_embodied_view_model` now renders rejection summaries/why-not details only when the `ValidationReport.actor_id` matches the current viewer.
- Embodied action availability provenance now extends from `report.actor_visible_facts`, not the unfiltered `report.checked_facts`; a source guard bans production `projections.rs` reads from `report.checked_facts.iter()`.
- The embodied "Knowledge context" render line now marks the global frontier as `DEBUG NON-DIEGETIC`, and existing leakage tests were narrowed to allow only that explicitly marked line.
- Added the 0021 possession-rebind conformance row and the `R-29 — Guard Vacuity / Decorative Locks` risk-register watch entry.

Deviations from original plan:

- The frontier-marking branch was chosen instead of deriving a viewer-scoped frontier.
- The core provenance test proves non-empty disabled-action provenance plus absence of the debug-only context-hash fact, while the source guard proves the exact actor-visible field source. The live disabled-action provenance shape does not expose a stable actor-visible fact string beyond the guardable source path.

Verification results:

- `cargo fmt --all --check` — passed.
- `cargo test -p tracewake-tui --test adversarial_gates` — passed.
- `cargo test -p tracewake-core projections` — passed.
- `cargo test -p tracewake-core --test anti_regression_guards checked_facts -- --nocapture` — passed.
- `cargo test -p tracewake-tui render` — initially exposed stale render expectations; after test updates, the relevant render and TUI acceptance coverage passed.
- `cargo test -p tracewake-tui --test embodied_flow` — passed.
- `cargo test -p tracewake-tui --test tui_acceptance` — passed.
- `cargo test --workspace` — passed.
