# 0039SPICERMUT-018: Kill `tui/render.rs` SPINE survivors with production-view-model render witnesses

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-tui` (test-only by default; a production correction in `tui/render.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 4 missed mutants in `crates/tracewake-tui/src/render.rs` (spec §5.17), owning SPINE-07 (TUI-first proof surface). The cluster flips embodied/notebook predicates and replaces an action-rejection rendering value with empty/`xyzzy` text. Prose snapshots may supplement but may not be the only oracle, and the rendering tests must travel from production view-model construction, not a test-only string path.

## Assumption Reassessment (2026-06-18)

1. `crates/tracewake-tui/src/render.rs` exists (verified present); §5.17 describes the surviving predicates (embodied/notebook section gating, action-rejection rendering) rather than naming individual function symbols — the 4 exact mutant identities are the seed work-list in `reports/0038_spine_cert_mutation_triage_register.md`, mapped at implementation time.
2. Spec §5.17 is the implementation contract; `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` and `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` govern the TUI proof surface (verified present).
3. Shared boundary under audit: the TUI render seam that turns production view models (built at a known event frontier) into embodied / notebook / rejection / debug sections.
4. Motivating invariants: `INV-067 — Embodied mode shows actor-known reality` and `INV-068 — Debug mode is visibly non-diegetic`.
5. This ticket touches the no-leak / embodied-rendering surface: ordinary embodied output must exclude debug-only tokens and truth-belief mismatch details; notebook content must appear only under its modeled availability rules; an action rejection must retain its typed reason and checked-fact ancestry through rendering; and each positive render must be paired with an adversarial model that must omit or quarantine the section. The witnesses only strengthen the embodied/debug split — no leakage is introduced — and there is no schema shape change (test additions, not a schema extension). This substrate feeds the SPINE-07 re-proof in ticket 021.

## Architecture Check

1. Rendering representative embodied/notebook/rejection/debug sections from real view models built at a known frontier, asserting semantic section presence + typed IDs/reasons/checked facts + channel markers, catches the predicate flips and empty/`xyzzy` rejection mutant — a prose snapshot alone does not.
2. No backwards-compatibility aliasing/shims: tests travel from production view-model construction; no test-only string path is instantiated.

## Verification Layers

1. INV-067 embodied fidelity -> TUI seam check: ordinary embodied output excludes debug-only tokens and truth-belief mismatch details; notebook content appears only under its modeled availability rules.
2. Rejection-ancestry rendering -> TUI seam check: an action rejection retains its typed reason and checked-fact ancestry through rendering (semantic section presence + typed IDs/reasons/checked facts + channel markers).
3. INV-068 adversarial omission -> adversarial gate: each positive render is paired with an adversarial model that must omit or quarantine the section.

## What to Change

### 1. Production-view-model render matrix

In `tui_seam_conformance.rs`, render representative embodied, notebook, rejection, and debug sections from real view models created at a known event frontier; assert semantic section presence, typed IDs/reasons/checked facts, and channel markers (prose snapshots supplement only).

### 2. Adversarial omission/quarantine rows

In `adversarial_gates.rs`, prove ordinary embodied output excludes debug-only tokens and truth-belief mismatch details, notebook content appears only under modeled availability, and a rejection retains its typed reason + checked-fact ancestry; pair each positive render with an adversarial model that must omit or quarantine the section.

### 3. Member matrix

Map all 4 historical mutants (plus any new run survivor in this file) to a concrete render-section or channel-marker consequence.

## Files to Touch

- `crates/tracewake-tui/tests/tui_seam_conformance.rs` (modify)
- `crates/tracewake-tui/tests/adversarial_gates.rs` (modify)
- `crates/tracewake-tui/src/render.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Implementation Disposition (2026-06-18)

Current-code reassessment confirmed the 4 historical `render.rs` survivors remained: embodied why-not fact gating, notebook empty-marker gating, and empty/`xyzzy` action-rejection rendering.

The implemented witnesses are test-only. `adversarial_gates.rs` now asserts production `TuiApp` rendering for typed notebook leads and action rejection output. `render.rs` unit tests add member-specific view-model rows for the why-not fact predicate and the typed-lead/no-empty-marker notebook branch, because the per-file mutation test slice for this package needed those exact render branches in unit coverage. No production rendering behavior changed.

## Out of Scope

- Transcript representative-section selection (ticket 019).
- Core view-model/debug-report internals (016/017).
- The full mutation run (ticket 020); SPINE re-proof (ticket 021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-tui --test tui_seam_conformance` — passes with the production-view-model render matrix asserting semantic sections + channel markers.
2. `cargo test --locked -p tracewake-tui --test adversarial_gates` — passes with the paired adversarial omission/quarantine rows.
3. `cargo mutants --workspace -f crates/tracewake-tui/src/render.rs --no-shuffle` — all 4 historical survivors (and any newly enumerated one) are `caught`.

### Invariants

1. Rendering tests travel from production view-model construction; no test-only string path, and prose snapshots are never the only oracle.
2. Embodied output excludes debug-only tokens and truth-belief mismatch; rejections retain typed reason + checked-fact ancestry.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/tui_seam_conformance.rs` — embodied/notebook/rejection/debug render sections from real view models with semantic + channel-marker assertions.
2. `crates/tracewake-tui/tests/adversarial_gates.rs` — paired adversarial models that must omit/quarantine debug-only and truth-belief-mismatch content.

### Commands

1. `cargo test --locked -p tracewake-tui --test tui_seam_conformance --test adversarial_gates`
2. `cargo mutants --workspace -f crates/tracewake-tui/src/render.rs --no-shuffle`
3. The per-file `-f` run is the correct verification boundary; the full standing campaign is ticket 020.

## Outcome

Completed: 2026-06-18

Added render witnesses that preserve typed why-not facts from an embodied view model, suppress the notebook empty marker when typed leads are present, and assert production `TuiApp` rendering preserves typed notebook lead fields and rejection summary/reason-code output. The adversarial surfaces still prove possession rebinding and debug rendering do not leak debug/non-diegetic facts into ordinary actor output.

Deviations from the original plan:

- `tui_seam_conformance.rs` remained unchanged; its existing seam checks passed. The new positive render assertions landed in `adversarial_gates.rs` and `render.rs` unit tests.
- The mutation command used `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-tui/src/render.rs --no-shuffle` instead of the ticket's bare command, because ticket 001 installed the standing `.cargo/mutants.toml`; `--no-config` preserves this ticket's per-file Wave B proof boundary.

Verification:

- `cargo test --locked -p tracewake-tui --lib render::tests` — passed, 11 render tests.
- `cargo test --locked -p tracewake-tui --test tui_seam_conformance --test adversarial_gates` — passed, 2 seam-conformance tests and 15 adversarial-gate tests.
- `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-tui/src/render.rs --no-shuffle` — passed with 14 mutants tested, 14 caught, 0 missed.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace --locked` — passed.
