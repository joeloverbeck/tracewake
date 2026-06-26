# 0053FOUCONFIF-009: Standing mutation campaign — full run + denominator record

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — evidence-only standing-campaign run; denominators handed to the capstone (010)
**Deps**: 0053FOUCONFIF-004, 0053FOUCONFIF-005, 0053FOUCONFIF-006, 0053FOUCONFIF-007

## Problem

Spec 0053 §8 / §7: after all code/test work, the implementing session must run the full standing mutation campaign and publish the selected denominator with the complete caught/missed/unviable/timeout disposition. The per-finding tickets run focused campaigns for fast feedback (notably 007's `cargo mutants -f projections.rs`); this ticket runs the configured standing campaign over the whole perimeter to completion and records the denominators that feed the §4.6 status manifest in the capstone (010). The canonical perimeter may not be called green until the §5 `food_source` family is resolved (closed by 007) — this run proves it.

## Assumption Reassessment (2026-06-26)

1. `.cargo/mutants.toml` exists (verified this session) and configures the standing perimeter; `.cargo/mutants-baseline-misses.txt` is currently empty (0 bytes). The CI mutation jobs `full-surface-mutation-trigger` (`.github/workflows/ci.yml:126`) and `mutants-lock-layer-reconcile` (366) drive the standing campaign. This ticket runs the campaign and records the disposition; it changes no config (the perimeter already covers the seams — §4.4 / §7).
2. Spec authority: `specs/0053_…_HARDENING_SPEC.md` §8 (run focused then full standing after all code/test work; publish denominators), §7 (mutation: focused for fast feedback, then configured standing campaign; canonical perimeter not green until §5 resolved), §9 step 7. Sibling precedent: the 0052 line ran the standing campaign and recorded `3400 selected / 2645 caught / 748 unviable / 7 routed-forward / 0 timeouts`; the seven were the `food_source` family this line closes (007), so a green in-surface result is expected.
3. Cross-artifact boundary under audit: the standing mutation perimeter (`.cargo/mutants.toml` + CI jobs) and the captured denominator disposition consumed by the capstone's §4.6 manifest (010). This is an evidence-only ticket — it reads the code the prior tickets resealed and records sensitivity.
4. Motivating invariant: INV-092 (deterministic replay is tested) and the INV-098-class evidence discipline — the standing campaign is the regression proof that the resealed surfaces and the closed survivor family hold under mutation, with no routed-forward residual left unrecorded.
5. This ticket *audits* the deterministic-replay / actor-knowledge / fail-closed-validation surfaces via mutation (evidence-consumer basis): it names them (the sealed bootstrap, the sealed interval product, the token-gated debug receipt, the food-source supersession rule) and confirms the campaign introduces no leakage/nondeterminism path — it runs deterministically against the configured perimeter and records caught/missed/unviable/timeout, feeding the manifest without altering any behavior. No schema shape change (item 6 N/A — evidence-only, no modify target).

## Architecture Check

1. A standalone `Engine Changes: None` run+record ticket is cleaner than folding the standing campaign into the capstone: the campaign is the expensive long-running step, so isolating it keeps the capstone a pure assembly+verdict, and its recorded denominators are a discrete input the manifest computes pass from. No `.cargo/mutants.toml` change is made here because the perimeter already covers the seams (§4.4 / §7).
2. No backwards-compatibility aliasing or shims (evidence-only; no code change).

## Verification Layers

1. INV-092 (mutation sensitivity) -> standing mutation campaign: the configured perimeter runs to completion; the disposition is recorded with zero in-surface misses/timeouts and an empty routed-forward set (the `food_source` family closed by 007).
2. Completeness -> manual review: the published denominator names selected/caught/missed/unviable/timeout, with every miss reconciled (killed by a finding ticket, or an explicit semantically-proven skip).
3. Cross-artifact: the recorded denominators map into the §4.6 manifest's mutation-evidence-status + survivor-list fields (consumed by 010).
4. Evidence-only ticket: the verification surface is the campaign run itself plus the recorded disposition; no new test logic is added.

## What to Change

### 1. Run the full standing campaign

From a clean baseline at the exact implementation commit, run the configured standing mutation campaign to completion (`cargo mutants` over the `.cargo/mutants.toml` perimeter, or the CI `full-surface-mutation-trigger` / `mutants-lock-layer-reconcile` lanes). Capture the complete caught/missed/unviable/timeout disposition and the selected denominator.

### 2. Record the denominators for the capstone

Hand the recorded disposition (selected, caught, missed, unviable, timeout, survivor list) to the capstone (010) for the §4.6 status manifest. Reconcile every miss: killed by a finding ticket, or carrying an explicit semantic-equivalence skip from 007. No routed-forward residual remains (the `food_source` family is closed); if any survivor is discovered, route it to a reserved follow-up ticket and record it as a bounded forcing-function residual in the manifest.

## Files to Touch

- `None — verification/evidence-only run; the standing-campaign denominators are captured and recorded in the capstone acceptance artifact (010). No config or source file is modified (the perimeter already covers the seams).`

## Out of Scope

- Any `.cargo/mutants.toml` perimeter change (the perimeter already covers the seams; §4.4 / §7).
- Killing specific survivors (007 owns the `food_source` family; any newly-discovered survivor routes to a reserved follow-up ticket).
- Authoring the acceptance artifact / computing pass (010).

## Acceptance Criteria

### Tests That Must Pass

1. The configured standing mutation campaign runs to completion at the exact implementation commit with zero in-surface misses and zero timeouts; the routed-forward set is empty (the `food_source` family closed by 007).
2. The published denominator (selected / caught / missed / unviable / timeout + survivor list) is captured and handed to the capstone manifest.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — clean baseline before the campaign.

### Invariants

1. Every mutation miss is reconciled (killed or semantically-proven skip); no routed-forward residual is left unrecorded (INV-092; §4.6.4).
2. The canonical standing perimeter is not called green unless the campaign confirms it (the `food_source` family resolved).

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; no test logic is added. The deliverable is the completed standing campaign and its recorded denominator disposition, consumed by the capstone (010).`

### Commands

1. `cargo mutants` — the configured standing campaign over the `.cargo/mutants.toml` perimeter (long-running; the correct verification boundary is the full campaign, not a narrow `-f`, since this ticket proves the whole perimeter; CI runs it via `full-surface-mutation-trigger` / `mutants-lock-layer-reconcile`).
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — clean baseline before the run.
3. This ticket runs no narrow command: its boundary is the full standing campaign to completion, which cannot be meaningfully dry-run; the recorded disposition is the deliverable.
