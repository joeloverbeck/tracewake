# 0056FOUCONSEV-006: Standing mutation campaign — run and record

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None
**Deps**: 0056FOUCONSEV-001, 0056FOUCONSEV-002, 0056FOUCONSEV-003, 0056FOUCONSEV-004, 0056FOUCONSEV-005

## Problem

Spec §7 (mutation) + §8 (acceptance evidence) + §5 (standing-mutation residual disposition). After the sealed-bootstrap (F7-01), debug-authority (F7-02), and taxonomy (F7-03/F7-04/F7-05) code/test work lands, the implementing session must run the focused per-surface campaigns then the full standing campaign from a clean baseline, and publish the selected denominator with the complete caught / missed / unviable / timeout disposition. This line carries no survivor family forward by default; the `food_source` family is static-present with a public actor-known witness and requires only preservation (no historical `0054` mutation count may be treated as current green evidence).

## Assumption Reassessment (2026-06-28)

1. The sealed surfaces under campaign are produced by the prior tickets: validated-content bootstrap (001, `state.rs`/`runtime/session.rs`/content files), debug-authority path (002, `debug_capability.rs`/`session.rs`/`command.rs`/TUI), and the taxonomy parser/guard functions (003/004 with perimeter wiring in 005). The standing campaign config is `.cargo/mutants.toml` + the CI `mutants-lock-layer` / `full-surface-mutation-trigger` lanes (verified at `2720167`).
2. Spec §5 + §7 + §8. The mutation perimeter is wired by 0056FOUCONSEV-005; this ticket *runs* it and records disposition — it changes no `.cargo/mutants.toml` (Engine Changes: None). Driver §7/§8.
3. **Boundary under audit**: the evidence boundary between the wired perimeter (005) and the recorded disposition consumed by the capstone (0056FOUCONSEV-009). This ticket produces the caught/missed/unviable/timeout denominators the capstone's acceptance artifact carries.
4. INV-092 (deterministic replay is tested) and INV-098 (harsh acceptance) — current mutation evidence, not a historical count, is what proves the sealed seams are behaviorally witnessed; a survivor on a sealed surface blocks a green perimeter claim.
5. **Evidence-consumer enforcement surface**: the campaign audits the fail-closed-acceptance, replay, and debug-quarantine surfaces sealed by 001–005 without modifying them. Item-5 on the evidence-consumer basis: confirm the run introduces no leakage/nondeterminism (it is read-only over the built tree) and that any survivor is killed through a public behavior witness or recorded under a bounded forcing function (§5) — naming owning surface, why not closed, the next execution move, a max remediation-epoch/concrete trigger, and the exact CI/mutation test that fails if it remains. No artifact may call the canonical standing perimeter green while any survivor remains.

## Architecture Check

1. A standalone run-and-record evidence ticket (no config change) is the correct shape: the perimeter is already configured (005), so this ticket's value is the executed campaign from a clean baseline and the honest disposition, which the capstone consumes. Folding it into the capstone would mix evidence generation with verdict rendering.
2. No backwards-compatibility shim — no code or config changes; the campaign runs the configured standing perimeter as-is.

## Verification Layers

1. INV-098 (harsh acceptance) -> the full standing `cargo mutants` campaign over the configured perimeter with published caught/missed/unviable/timeout disposition; zero survivors on the sealed surfaces, or each recorded under a §5 bounded forcing function.
2. INV-092 (replay tested) -> the focused per-surface campaigns (bootstrap, debug-authority, taxonomy guards) complete with current disposition.
3. Evidence-only ticket — the proof surface is the recorded campaign disposition; no new production logic.

## What to Change

### 1. Run the focused per-surface campaigns

For fast feedback during/after implementation: `cargo mutants -f` over the bootstrap (001), debug-authority (002), and taxonomy-guard (003/004/005) surfaces.

### 2. Run the full standing campaign from a clean baseline

After all code/test work lands, run the configured standing campaign to completion; publish the selected denominator and the complete caught/missed/unviable/timeout disposition.

### 3. Record residual disposition

Record any survivor under a §5 bounded forcing function, or kill it via a public behavior witness; confirm the `food_source` witness is preserved and no historical `0054` count is treated as current evidence.

## Files to Touch

- `None — evidence-only run; the caught/missed/unviable/timeout disposition is recorded in this ticket's evidence and consumed by the 0056FOUCONSEV-009 acceptance artifact.`

## Out of Scope

- Wiring the mutation perimeter or CI lanes (0056FOUCONSEV-005).
- Authoring new witnesses for surfaces already witnessed by 001–004 — this ticket runs and records; a newly-discovered survivor needing a fix routes to a reserved follow-up ticket or a bounded forcing function.
- Rendering the acceptance verdict (0056FOUCONSEV-009).

## Acceptance Criteria

### Tests That Must Pass

1. The full standing `cargo mutants` campaign completes from a clean baseline with a published denominator and complete disposition.
2. Zero survivors on the sealed surfaces (validated-content bootstrap, debug-authority path, taxonomy parser/guard functions), or each survivor recorded under a §5 bounded forcing function with its failing CI/mutation test named.

### Invariants

1. No artifact may call the canonical standing perimeter green while any survivor remains; no `mutation_status` computes pass with survivors present.
2. The historical `0054` mutation count is not carried forward as current green evidence; the `food_source` actor-known witness is preserved.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; verification is the configured standing mutation campaign and existing perimeter coverage named in Assumption Reassessment.`

### Commands

1. `cargo mutants --in-diff` — focused in-diff campaign for fast feedback on each sealed surface.
2. `cargo mutants` — full standing campaign over the configured perimeter from a clean baseline; capture the caught/missed/unviable/timeout disposition.
3. `cargo test --workspace` — confirm the clean baseline before the standing run (mutation evidence is only meaningful over a green tree).
