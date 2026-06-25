# 0051FOUCONTHI-012: Capstone — acceptance artifact and per-finding closure evidence

**Status**: COMPLETE
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — acceptance-only; assembles the spec `0051` §7 acceptance artifact from the upstream tickets' evidence. No new production logic.
**Deps**: 0051FOUCONTHI-010, 0051FOUCONTHI-011

## Problem

Spec `0051` §7 requires an acceptance artifact that begins from a clean baseline, names the exact implementation commit, runs the four gate commands, reproduces the preserved focused mutation commands, runs the full standing campaign, and records per-finding closure with real production-path evidence (production constructor, public command, observed effect, sensitivity proof) — and must not call the perimeter green before the standing campaign. This capstone exercises the pipeline the `-001`..`-011` tickets composed and renders that verdict; it introduces no new production logic.

## Assumption Reassessment (2026-06-24)

1. Codebase/docs: the acceptance-artifact template is `docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md`; the repo convention places acceptance artifacts at `reports/<NNNN>_<slug>_acceptance.md` (e.g. `reports/0048_foundational_conformance_hardening_acceptance.md`). The upstream tickets `-002`..`-010` produce the per-finding code/evidence this artifact cites; `-011` truths the docs.
2. Specs/docs: spec `0051` §7 (acceptance artifact and evidence), §3 (determination), §4.10 (preserved properties to re-confirm). The artifact is archived to `archive/reports/` on spec acceptance (`docs/archival-workflow.md`).
3. Shared boundary under audit: the end-to-end production path — every closure claim must trace to a production constructor, a public command, an observed state/event/projection effect, and a sensitivity proof (mutation or negative-compile).
4. INV-098 (feature acceptance is harsh): restated — a runnable feature is done only when caused, agent-possible, eventful, trace-aware, epistemically bounded, TUI-playable, debug-inspectable, no-human-runnable, replay-safe, LLM-independent, non-scripted, and regression-tested; the perimeter is not green before the standing campaign.
5. Fail-closed / replay / actor-knowledge surface (evidence-consumer basis): the artifact records re-run results over the no-human, replay, and embodied-leak surfaces; it asserts no green/red command result of its own beyond the re-runs and introduces no leakage path (debug rows stay observer-only).

## Architecture Check

1. A single acceptance-only capstone that re-runs every gate and renders the verdict (rather than per-finding self-certification) is the §7 shape and keeps the green/red claim in one auditable place gated on the standing campaign. It adds no production logic — it exercises the pipeline `-001`..`-011` composed.
2. No backwards-compatibility alias: N/A (evidence artifact only).

## Verification Layers

1. INV-098 (harsh acceptance) -> the artifact maps each F-01..F-09 closure to a production constructor + public command + observed effect + sensitivity proof, with the four gate commands and the standing-campaign disposition.
2. Determinism/replay (INV-018/092) -> the artifact records the continuation-equivalence and replay-identity re-runs (`-003`/`-004`) at reassessment time.
3. No-leak (INV-093) -> the artifact records the embodied-leak-absence + information-flow differential results (`-008`).

## What to Change

### 1. Assemble the acceptance artifact

Author `archive/reports/0051_foundational_conformance_third_hardening_acceptance.md` following `0003_ACCEPTANCE_ARTIFACT_TEMPLATE`. Begin from a clean baseline; name the exact implementation commit (not the proposal baseline `0429a8f`); record the four gate commands (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`); reproduce the preserved focused mutation commands; record the full standing `cargo mutants` disposition (selected denominator + caught/missed/unviable/timeout); and record per-finding closure with real production-path evidence.

### 2. Re-confirm preserved properties

Record the §4.10 preserved-property re-runs (atomic scratch-state cutover, duration completion, single-charge need accounting, body-exclusive reservation, empty-tick ancestry, replay fail-closed on temporal divergence, duplicate-`EventId` rejection, positive holder-known interval construction) under the core-owned session.

## Files to Touch

- `archive/reports/0051_foundational_conformance_third_hardening_acceptance.md` (new)

## Out of Scope

- Any production-logic change (this is acceptance-only; fixes route back to the owning ticket).
- The `docs/4-specs/SPEC_LEDGER.md` archived row and the `archive/specs/` move (deferred to spec acceptance — Step 6 cross-spec follow-up).
- Calling the perimeter green before the standing campaign completes.

## Acceptance Criteria

### Tests That Must Pass

1. The artifact records all four gate commands passing on the clean implementation tree with the named implementation commit.
2. The artifact records the full standing `cargo mutants` disposition and the reproduced focused commands; no green claim precedes the standing campaign.
3. Each of F-01..F-09 maps to a production constructor + public command + observed effect + sensitivity proof, and the §4.10 preserved properties are re-confirmed.

### Invariants

1. The capstone introduces no new production logic.
2. The verdict is rendered in one auditable artifact, gated on the standing campaign.

## Test Plan

### New/Modified Tests

1. `None — verification-only; the capstone re-runs existing gate/mutation/replay/leak commands and records their results in the acceptance artifact.`

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
2. `cargo mutants --timeout 183` (standing config — disposition recorded in the artifact)
3. `cargo test -p tracewake-core --test acceptance_artifact_wording` (artifact-wording guard, if present)

## Outcome

Completed: 2026-06-24

Outcome amended: 2026-06-24

Created `archive/reports/0051_foundational_conformance_third_hardening_acceptance.md`
as the single scoped acceptance artifact for spec `0051`.

The artifact was built from a clean temporary worktree at exact implementation
commit `a3b46c6f3f3cbd4ea02ec685a42d2d61be7ce953` and records:

1. The four clean-worktree gates:
   - `cargo fmt --all --check` -> passed.
   - `cargo clippy --workspace --all-targets -- -D warnings` -> passed.
   - `cargo build --workspace --all-targets --locked` -> passed.
   - `cargo test --workspace` -> passed.
2. Eight clean-worktree focused mutation campaigns:
   - 0051 projection focus: `6 mutants tested in 82s: 6 caught`.
   - Preserved 0049/0050 focused commands: all completed with zero missed
     selected mutants; unviables recorded per command.
3. Clean-worktree standing mutation disposition:
   - `cargo mutants --timeout 183` -> `3275 mutants tested in 3h: 23 missed,
     2549 caught, 703 unviable`.
   - Artifact counts: `2549 caught`, `23 missed`, `703 unviable`, `0 timeout`.
   - The artifact explicitly does not call the standing perimeter green.
4. Per-finding closure for F-01..F-09, each mapped to production constructor,
   public command/boundary, observed effect, and sensitivity proof.
5. §4.10 preserved-property reconfirmation covering atomic cutover, duration
   completion, need accounting, reservation, empty-tick ancestry, temporal
   divergence replay failure, duplicate `EventId` rejection, and positive
   holder-known interval construction.

Additional verification:

1. `cargo test -p tracewake-core --test acceptance_artifact_wording` -> passed.
2. `rg -n "Phase 1 / Phase 1A third hardening|Exact implementation commit|3275 mutants|23 missed|F-0[1-9]|LoadedWorldRuntime::from_loaded_world|Forbidden wording" archive/reports/0051_foundational_conformance_third_hardening_acceptance.md`
   -> confirmed the artifact contains the required scoped wording and evidence
   anchors.
3. `git diff --check` -> passed.
