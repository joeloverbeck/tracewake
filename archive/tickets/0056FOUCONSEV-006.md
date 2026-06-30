# 0056FOUCONSEV-006: Standing mutation campaign — run and record

**Status**: COMPLETE
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

### 2. Run one full standing discovery campaign from a clean baseline

After all code/test work lands, run the configured standing campaign to completion; publish the selected denominator and the complete caught/missed/unviable/timeout disposition.

Do **not** stop and restart the full campaign at the first survivor. A survivor is evidence, not an interrupt condition. Let the run finish so the ticket captures the whole missed/timeout set in one pass.

### 3. Record residual disposition

Record any survivor under a §5 bounded forcing function, or kill it via a public behavior witness; confirm the `food_source` witness is preserved and no historical `0054` count is treated as current evidence.

When survivors are found, batch them by owning surface and remediate them together. Use focused mutation commands and `cargo mutants --iterate` against the existing `mutants.out` state to converge on the remaining missed/time-out set instead of rerunning already-caught mutants from scratch after every fix. Preserve the discovery-run disposition in the ticket evidence.

Run a final full standing campaign from a clean baseline only after the survivor set is empty or all residuals have been explicitly bounded under §5. This final pass is the only campaign that may be cited as canonical green evidence.

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
2. `cargo mutants` — full standing discovery campaign over the configured perimeter from a clean baseline; let it complete even if `missed.txt` becomes non-empty.
3. `cargo mutants --iterate` — after batched survivor fixes, reuse the prior `mutants.out` state to focus on remaining missed/time-out work rather than restarting the full denominator.
4. `cargo mutants` — final clean-baseline standing campaign after convergence; this is the canonical green proof if and only if no live survivor remains.
5. `cargo test --workspace` — confirm the clean baseline before standing runs (mutation evidence is only meaningful over a green tree).

## Outcome

Completed: 2026-06-30

The standing mutation campaign was executed to current evidence instead of carrying forward the historical `0054` count. The campaign initially discovered one survivor after previously classified mutants were reused:

- `cargo mutants --workspace --no-shuffle --iterate` at `5d3f934`: `1571 mutants tested in 3h: 1 missed, 1192 caught, 378 unviable`; `previously_caught.txt` carried 2044 prior caught/unviable classifications, for a 3615-mutant disposition. `timeout.txt` was empty.
- Survivor: `crates/tracewake-core/src/runtime/session.rs:242:9: replace LoadedWorldRuntime::actor_exists -> bool with true`.
- Fix commit: `37062d6 Kill actor exists mutation survivor`, adding a public runtime witness that known actors return true and absent actors return false.

Focused verification after the survivor fix:

- `cargo test --locked -p tracewake-core runtime::session::tests::actor_exists_reports_known_and_absent_actors` passed.
- `cargo fmt --all --check` passed.
- `cargo mutants --no-config --file crates/tracewake-core/src/runtime/session.rs -F actor_exists --list` listed the two expected `actor_exists` mutants.
- `cargo mutants --no-config --file crates/tracewake-core/src/runtime/session.rs -F actor_exists` passed: `2 mutants tested in 2m: 2 caught`.

The full standing campaign was not rebooted after the survivor. The existing `mutants.out` state was reused:

- `cargo mutants --workspace --no-shuffle --iterate` at `37062d6`: `INFO Iteration excludes 3614 previously caught or unviable mutants`; `Found 1 mutant to test`; result `1 mutant tested in 2m: 1 caught`.
- Convergence disposition: `caught.txt` 1, `missed.txt` 0, `unviable.txt` 0, `timeout.txt` 0, `previously_caught.txt` 3614.

Final canonical clean standing campaign:

- Command: `cargo mutants --workspace --no-shuffle` from a clean temp worktree at `37062d6`.
- Result: `3451 mutants tested in 6h: 2681 caught, 770 unviable`.
- Disposition files: `caught.txt` 2681, `missed.txt` 0, `unviable.txt` 770, `timeout.txt` 0.

No survivor remains on the sealed surfaces, no timeout residual remains, and no §5 bounded forcing function is needed. The `food_source` witness remained in the current clean campaign perimeter and no historical `0054` mutation count is used as current green evidence.
