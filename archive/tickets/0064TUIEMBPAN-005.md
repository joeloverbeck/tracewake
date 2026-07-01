# 0064TUIEMBPAN-005: Embodied pane layout acceptance artifact

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None
**Deps**: 0064TUIEMBPAN-003, 0064TUIEMBPAN-004

## Problem

Spec 0064 §7 requires a single review artifact recording the fixed-size pane/buffer snapshots, the actor-known-only negative, the overflow/collapse tests, and the non-vacuity witnesses, captured at the exact implementation commit. §6 enumerates the witness suite. This capstone assembles that evidence end-to-end after the implementation tickets land; it introduces no new production logic and exercises the pipeline the earlier tickets composed (INV-095: TUI/view-model tests are acceptance tests).

## Assumption Reassessment (2026-07-02)

1. This is an acceptance-only capstone: it adds no production code and no new test logic — it runs the suites authored by 0064TUIEMBPAN-002 (actor-known-only negative), 0064TUIEMBPAN-003 (fixed-size buffer/text goldens + parity), and 0064TUIEMBPAN-004 (narrow-collapse, overflow truncation, non-vacuity, floor), and records their witnesses. The acceptance-artifact path convention is `reports/<NNNN>_<slug>_acceptance.md` (e.g. `reports/tui-experience-overhaul-research-report.md` and the archived `archive/reports/0063_core_actor_known_co_present_activity_acceptance.md` establish the `reports/` home; archived to `archive/reports/` on spec acceptance).
2. The witness suite is fixed by spec §6/§7 in `specs/0064_TUI_EMBODIED_PANE_LAYOUT_AND_AT_A_GLANCE_PANELS_SPEC.md`. Deps `003` and `004` are the leaf set — `003` transitively covers `001`/`002`, `004` covers `001`/`002` — so together they gate every implementation surface this artifact records. Re-enumerate expected snapshot sizes and counts from the fixtures at capture time rather than hardcoding.
3. Shared boundary under audit: this ticket reads/exercises the actor-known-only projection (002), the deterministic buffer render (003), and the responsive/non-vacuity behavior (004). It modifies none of them; it consumes their evidence.
4. Constitutional invariant under audit: **INV-095 — TUI/view-model tests are acceptance tests** — this artifact is the acceptance evidence for the phase, mapped to the fixed-size snapshots + non-vacuity witnesses (spec §10). **INV-093 — Actor-knowledge leakage is a high-severity defect** motivates recording the actor-known-only negative as a first-class witness.
5. Evidence-consumer basis for the epistemic/replay surfaces: this capstone audits (does not modify) the actor-knowledge filter (embodied panes carry no debug token) and the deterministic snapshot behavior (identical inputs → byte-identical goldens, INV-018 / spec §8). Confirm the recorded evidence introduces no leakage path and that the artifact's rows stay observer-only (no hidden truth captured into the report). No production enforcement surface changes here.

## Architecture Check

1. A single trailing acceptance artifact keeps the phase's exit evidence in one reviewable place, captured at the implementation commit, rather than scattering acceptance claims across implementation tickets. It exercises the composed pipeline without adding logic, so it cannot mask a real regression behind capstone-only code.
2. No backwards-compatibility aliasing or shims: verification-only; no production or test logic is introduced, only the evidence artifact.

## Verification Layers

1. INV-095 (TUI/view-model acceptance) -> replay/golden-fixture check: the full `0064` witness suite (002/003/004 tests) passes at the recorded commit and the artifact cites each.
2. INV-093 / INV-024 (no leakage) -> manual review: the recorded panes/snapshots contain no debug/non-diegetic token; the actor-known-only negative is captured green.
3. INV-018 (determinism) -> replay/golden-fixture check: re-running the goldens is byte-stable; the artifact records the deterministic-render confirmation.

## What to Change

### 1. Author the acceptance artifact

Add `reports/0064_tui_embodied_pane_layout_acceptance.md` recording, at the exact implementation commit: the fixed-size pane/buffer snapshots (`80x24`/`100x30`/`60x20`), the actor-known-only negative result, the overflow and narrow-collapse results, the non-vacuity witness, the buffer/text parity result, and the deterministic-render confirmation — each with the command run and its pass/fail, plus the scoped-verdict boilerplate matching the sibling `0063` acceptance report.

## Files to Touch

- `reports/0064_tui_embodied_pane_layout_acceptance.md` (new)

## Out of Scope

- Any production or test-logic change (owned by 0064TUIEMBPAN-001..004).
- The `docs/4-specs/SPEC_LEDGER.md` archived-row entry and the `archive/specs/` move — deferred to spec acceptance/archival per `docs/archival-workflow.md` (spec 0064 §0 Ledger timing: "no ledger row now").
- The §5 doctrine amendment (Architecture 13 / Execution 10) — routed to Spec 0070, not ratified here.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-tui` — the full `0064` witness suite (pane_bindings actor-known-only negative, embodied_pane_buffer goldens + parity, embodied_pane_responsive collapse/overflow/non-vacuity/floor) passes.
2. `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --all --check` — full gate green at the capture commit.
3. The artifact records every spec §6 witness with its command and pass/fail and cites the exact commit.

### Invariants

1. The artifact is observer-only evidence — it records witnesses and contains no hidden truth or debug leakage (INV-093/INV-107).
2. Recorded goldens are deterministic and reproduced byte-identically at review (INV-018).

## Test Plan

### New/Modified Tests

1. `None — acceptance/evidence-only ticket; verification runs the existing 002/003/004 suites and records witnesses. No new test logic is introduced.`

### Commands

1. `cargo test -p tracewake-tui`
2. `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --all --check`
3. A narrower per-suite command is not the correct boundary here: the capstone's contract is the whole-phase witness set, so the full TUI-crate test run plus the workspace gate is the acceptance boundary.

## Outcome

Completed: 2026-07-02

Added `reports/0064_tui_embodied_pane_layout_acceptance.md`, recording the
Spec 0064 witness matrix, fixed-size pane text goldens, `ratatui` buffer
snapshots, actor-known-only negative, overflow/collapse/non-vacuity/floor
witnesses, buffer/text parity, deterministic-render checks, and scoped verdict
for implementation commits `edeffdb`, `9f49307`, `84c8b7e`, and `e699dbb`.

Verification run:

- `cargo test -p tracewake-tui`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`

No production code or test logic was added by this ticket; it is evidence-only.
The acceptance report remains live under `reports/` until final spec closeout
archives the report and retargets references.
