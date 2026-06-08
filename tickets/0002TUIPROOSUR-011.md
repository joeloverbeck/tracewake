# 0002TUIPROOSUR-011: Deterministic typed transcript and replay stability

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (canonical ordering of rendered-for-test view-model lists), `tracewake-tui` (transcript)
**Deps**: 0002TUIPROOSUR-002, 0002TUIPROOSUR-005, 0002TUIPROOSUR-010

## Problem

Tickets 002–010 reshape the embodied view model, availability, why-not, and notebook surfaces. Spec 0002 §4 TUI-AC-010 and §5 PSL-004 require that TUI transcripts stay deterministic, that acceptance be based on typed view-model/proposal/report/replay artifacts first and rendered transcript snapshots second, and that all rendered-for-test lists use ordered collections / canonical sorting (not incidental hash-map order). Debug panels must remain provably read-only — they may not mutate the event log, physical/agent checksum, controller binding (except explicit bind), or embodied context.

## Assumption Reassessment (2026-06-08)

1. Deterministic transcript tests exist (`crates/tracewake-tui/tests/tui_acceptance.rs:26-33`; `crates/tracewake-tui/tests/transcript_snapshot.rs:6-33`); replay rebuild applies ordered event streams and validates physical/agent checksums (`crates/tracewake-core/src/replay/rebuild.rs:45-91,491-540`). The transcript module is `crates/tracewake-tui/src/transcript.rs`.
2. Spec 0002 §4 TUI-AC-010: ordered collections / canonical sorting for all rendered-for-test lists; keep the stdout/scripted runner available; debug panels cannot mutate event log/checksums/binding/context.
3. Cross-artifact boundary under audit: the rendered view-model lists (core `projections.rs`/`view_models.rs`, reshaped by 002/005/010) ↔ the transcript (`tracewake-tui`) ↔ replay rebuild (core). The ordering contract must hold across all three.
4. Invariants restated: **INV-017** — auditable/seedable determinism; **INV-018** — deterministic replay reconstructs outcomes byte-identically; **INV-092** — deterministic replay is tested; **INV-095** — TUI/view-model tests are acceptance tests.
5. Fail-closed / replay surface: confirm every rendered-for-test list (semantic actions, availability facts, notebook leads, why-not facts) uses an ordered/sorted collection so identical inputs+versions yield identical transcripts; and assert debug panels are read-only (no mutation of event log, physical/agent checksum, controller binding except explicit bind, or embodied context). This is the determinism regression-lock after the view reshape; it changes ordering, not replay/hash *semantics*, so no foundational-doc amendment is required.

## Architecture Check

1. Canonicalizing list order at the view-model layer makes transcript determinism a property of the data, not of the renderer or map iteration, so a rendering or collection change cannot silently perturb replay/transcript outcomes (INV-018) — and lets semantic tests assert typed artifacts independently of rendered text (PSL-004).
2. No backwards-compatibility aliasing/shims: ordering is fixed at the source collections; no parallel "sorted view for tests" shim is introduced.

## Verification Layers

1. INV-018/INV-092 (deterministic replay) -> replay/golden-fixture check: replay from fixture + ordered log reproduces matching physical and agent checksums after the view reshape.
2. INV-017 (determinism) -> behavior test: repeated representative embodied/debug/possession/no-human transcripts produce identical typed artifacts and identical rendered transcript.
3. INV-095 (read-only debug) -> codebase grep-proof / test: debug panels take `&` references and a test asserts checksums/binding/context are unchanged after opening every debug panel.

## What to Change

### 1. Canonical ordering of rendered-for-test lists

Ensure semantic-action lists, availability/why-not fact sets, and notebook leads use ordered/canonically-sorted collections at the view-model layer.

### 2. Debug read-only guarantee

Assert (via `&`-only signatures + a test) that debug panels cannot mutate the event log, checksums, controller binding (except explicit bind), or embodied context.

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/view_models.rs` (modify)
- `crates/tracewake-tui/src/transcript.rs` (modify)

## Out of Scope

- Adding a second/visual renderer or `ratatui`/`crossterm` (Spec 0002 §6 — deferred).
- The adversarial transcript gates and acceptance capstone (ticket 013).
- Replay/hash *semantics* (unchanged; only collection ordering is canonicalized).

## Acceptance Criteria

### Tests That Must Pass

1. Repeating representative embodied/debug/possession/no-human transcripts twice yields identical typed artifacts and identical rendered transcript.
2. Replay from initial fixture + ordered log reproduces matching physical and agent checksums; debug-panel opening mutates nothing.
3. `cargo test -p tracewake-core && cargo test -p tracewake-tui` pass; `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. Rendered-for-test lists are canonically ordered; transcripts are deterministic (INV-017/INV-018).
2. Debug panels are read-only with respect to world state (INV-095).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/transcript_snapshot.rs` — double-run determinism over the reshaped views.
2. `crates/tracewake-core/tests/golden_scenarios.rs` — replay checksum match post-reshape; debug read-only assertion.

### Commands

1. `cargo test -p tracewake-tui transcript_snapshot`
2. `cargo test -p tracewake-core golden_scenarios`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
