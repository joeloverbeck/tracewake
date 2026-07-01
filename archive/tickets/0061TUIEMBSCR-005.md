# 0061TUIEMBSCR-005: Fixtures, goldens, and acceptance artifact (capstone)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds golden fixtures and an acceptance test harness for the embodied screen dumps.
**Deps**: 0061TUIEMBSCR-002, 0061TUIEMBSCR-003, 0061TUIEMBSCR-004

## Problem

Spec 0061 §6–§7 require the seam's acceptance evidence: fixed-size plain-text dump goldens, a
structured `ScreenDump` snapshot for an embodied fixture, the field-disposition conformance result,
a debug-token-absence negative, and a determinism (byte-identical) check — all assembled into a
review artifact recording them at the exact implementation commit, with screen dumps declared
first-class review artifacts. This capstone ticket adds the golden/snapshot harness exercising the
dumps (002/003) and the disposition guard (004) end-to-end, and authors the acceptance artifact. It
introduces no new production logic; it exercises and locks the pipeline the prior tickets composed.

## Assumption Reassessment (2026-07-01)

1. The dumps under test are 002's `render_embodied_screen_dump` and 003's `ScreenDump` projection,
   over 001's `EmbodiedScreenModel`; the disposition guard is 004's
   `embodied_screen_model_field_disposition`. The embodied fixture is `ordinary_workday_001`
   (`crates/tracewake-content/src/fixtures/ordinary_workday_001.rs`), already used by the TUI
   (`crates/tracewake-tui/src/launch.rs`). Goldens live under the existing
   `crates/tracewake-tui/tests/goldens/` directory.
2. Spec 0061 §6 names the fixtures/tests (goldens at `80x24`, `100x30`, `60x20`; a structured
   `ScreenDump` snapshot; the disposition conformance; the debug-token-absence negative; the
   determinism check); §7 names the review artifact and its contents.
3. Shared boundary under audit: the end-to-end dump pipeline (view model → `EmbodiedScreenModel` →
   plain-text/structured dumps) and its golden/snapshot locks. This ticket consumes 002/003/004 and
   produces the acceptance artifact; it modifies none of their source.
4. Invariants motivating this ticket: **INV-018 — Deterministic replay is foundational** (repeated
   build + dump is byte-identical), **INV-024 — No telepathy** (the no-leak negative), and
   **INV-068 — Debug mode is visibly non-diegetic** (the debug-token-absence negative). Acceptance
   framing follows **INV-095 — TUI/view-model tests are acceptance tests**.
5. Fail-closed / actor-knowledge + replay surface: the enforcement surfaces are embodied-output
   actor-knowledge filtering (INV-024/INV-067) and dump determinism (INV-018). This ticket proves
   them by test — the debug-token-absence negative asserts no `DEBUG NON-DIEGETIC` token in any
   embodied dump, and the determinism test asserts byte-identical repeated build+dump on the same
   view model. It adds only tests/goldens/artifact; it introduces no runtime path and weakens no
   enforcement.

## Architecture Check

1. Stable pane text dumps are the primary golden and the structured `ScreenDump` snapshot is the
   secondary machine-parseable lock (Spec 0061 §9.1 #6) — locking both from one fixture proves the
   "two dumps, one source" guarantee at the artifact level, not just in unit tests. Re-enumerating
   expected panes/fields from the fixture at test start (rather than hardcoding counts) keeps the
   goldens from going stale as the fixture evolves.
2. No backwards-compatibility aliasing/shims: this is verification-and-evidence only; it adds no
   production code and touches no prior ticket's source.

## Verification Layers

1. INV-018 (determinism) -> replay/golden check: repeated `build_embodied_screen_model` + both dumps
   on `ordinary_workday_001` are byte-identical; the fixed-size plain-text goldens match.
2. INV-024/INV-068 (no-leak, debug non-diegetic) -> manual-review + negative test: no embodied dump
   (plain-text or structured per-pane) contains a `DEBUG NON-DIEGETIC` token.
3. INV-095 (acceptance artifact) -> the review artifact records the goldens, the structured snapshot,
   the field-disposition conformance result, and the debug-token-absence negative at the
   implementation commit.

## What to Change

### 1. Golden + snapshot harness

Add `crates/tracewake-tui/tests/embodied_screen_dump.rs` exercising `ordinary_workday_001`:
fixed-size plain-text dump goldens at `80x24`, `100x30`, and `60x20`; a structured `ScreenDump`
snapshot; a debug-token-absence negative over every embodied dump; and a determinism test asserting
byte-identical repeated build + dump.

### 2. Golden fixtures

Add the fixed-size plain-text golden files under `crates/tracewake-tui/tests/goldens/` and the
structured `ScreenDump` snapshot file.

### 3. Acceptance artifact

Author `reports/0061_tui_embodied_screen_model_acceptance.md` recording the fixed-size dump goldens,
the structured `ScreenDump` snapshot, the field-disposition conformance result (004), and the
debug-token-absence negative — declaring screen dumps first-class review artifacts (Spec 0061 §7).

## Files to Touch

- `crates/tracewake-tui/tests/embodied_screen_dump.rs` (new)
- `crates/tracewake-tui/tests/goldens/embodied_screen_80x24.txt` (new)
- `crates/tracewake-tui/tests/goldens/embodied_screen_100x30.txt` (new)
- `crates/tracewake-tui/tests/goldens/embodied_screen_60x20.txt` (new)
- `crates/tracewake-tui/tests/goldens/embodied_screen_dump.snapshot.txt` (new — structured `ScreenDump` snapshot)
- `reports/0061_tui_embodied_screen_model_acceptance.md` (new)

## Out of Scope

- The screen model, builder, and dump renderers (001/002/003) and the disposition guard (004) —
  exercised here, not modified.
- The `docs/4-specs/SPEC_LEDGER.md` archived row and the `archive/specs/` move — deferred to spec
  acceptance per Spec 0061 §0 (feature-spec archival convention), not part of this ticket.
- Any `ratatui`/`crossterm` buffer path (Spec 0064/0065) or raw-buffer visual review.

## Acceptance Criteria

### Tests That Must Pass

1. Plain-text dump goldens match at all three fixed sizes — `80x24`, `100x30`, `60x20` — for
   `ordinary_workday_001` (expected panes re-enumerated from the fixture, not hardcoded).
2. The structured `ScreenDump` snapshot for `ordinary_workday_001` matches its locked form.
3. No embodied dump (plain-text or any structured per-pane dump) contains a `DEBUG NON-DIEGETIC`
   token.
4. Repeated `build_embodied_screen_model` + both dumps on the same view model are byte-identical.
5. `reports/0061_tui_embodied_screen_model_acceptance.md` records all four evidence items.

### Invariants

1. Identical inputs produce byte-identical dumps (determinism).
2. Embodied dumps never carry a debug non-diegetic marker.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/embodied_screen_dump.rs` — fixed-size goldens, structured snapshot,
   debug-token-absence negative, and determinism.

### Commands

1. `cargo test -p tracewake-tui --test embodied_screen_dump`
2. `cargo test -p tracewake-tui`
3. `cargo test --workspace` (full-pipeline confirmation that the seam and its acceptance evidence
   hold alongside the existing TUI/view-model acceptance tests).

## Outcome

Completed: 2026-07-01

Added the `ordinary_workday_001` embodied screen-dump capstone test harness,
fixed-size plain-text dump goldens at `80x24`, `100x30`, and `60x20`, a
structured `ScreenDump` snapshot, and the acceptance report
`reports/0061_tui_embodied_screen_model_acceptance.md`. The test proves
byte-identical repeated screen-model build plus plain-text and structured
dumps, and verifies no embodied dump carries `DEBUG NON-DIEGETIC`.

The first `cargo test --workspace` run exposed a required adjacent guard update:
the existing production-source census did not yet classify the new
`crates/tracewake-tui/src/screen/*.rs` files. The capstone adds those files to
the existing TUI source classification table in
`crates/tracewake-core/tests/anti_regression_guards.rs`, then reran the failed
guard and the full required commands.

Verification after the report existed:

- `cargo test -p tracewake-tui --test embodied_screen_dump` passed.
- `cargo test -p tracewake-tui` passed.
- `cargo test --workspace` passed.

Additional corrective verification:

- `cargo test -p tracewake-core --test anti_regression_guards workspace_source_classification_census_matches_production_tree` passed after the census update.

Deviations:

- None.
