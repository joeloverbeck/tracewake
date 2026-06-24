# 0050FOUCONSEC-011: Focused + standing mutation campaigns

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None
**Deps**: 0050FOUCONSEC-010

## Problem

Spec-0050 §6.3 + §8 (closure step 8): after the remediation lands, run focused `cargo-mutants` campaigns over the new branches, **preserve and rerun** the three `0049` focused commands, then run the configured standing perimeter, recording exact denominators and caught/missed/unviable sets. The narrow campaigns are not a replacement for the standing lane. **No `.cargo/mutants.toml` change** — the perimeter already covers the seams (spec §3.2 / §6.3, verified: `.cargo/mutants.toml` present and covers scheduler/replay/projection/pipeline/TUI).

## Assumption Reassessment (2026-06-24)

1. `.cargo/mutants.toml` exists and already includes the scheduler/replay/projection/pipeline/TUI seams (verified at `HEAD` `8d7c119`; spec §3.2 records the perimeter as already-covering). The three `0049MUTWIT` focused commands are recorded in `archive/tickets/0049MUTWIT-001..003.md`. This ticket runs and records; it changes no config and no source.
2. Spec-0050 §6.3 is authoritative: no perimeter expansion; the delta is behavior that kills the new branches' mutants, plus rerunning the preserved `0049` commands and the standing lane.
3. Shared boundary under audit: the mutation evidence boundary — the standing `.cargo/mutants.toml` perimeter and the focused campaigns over the functions changed by `-001`…`-009`. This is an evidence-collection ticket; it authors no production code.
4. `INV-092` (deterministic replay tested) and `INV-098` (harsh feature acceptance, regression-tested) motivate this ticket: surviving mutants signal a missing witness or an equivalent mutant; closure records the denominator/artifact, not a declaration.
5. Enforcement surface (evidence-consumer basis): the campaigns audit the new discovery/process/actor-outcome/`EventId`/replay-report/salience branches. Running mutation introduces no production behavior, no replay/leak path; it measures whether the `-001`…`-010` witnesses kill the relevant mutants. Reproduce the CI mutation environment (flags/versions) before trusting caught/missed/unviable counts.

## Architecture Check

1. Running focused campaigns during/after each change plus the standing perimeter at closeout — rather than expanding the config — matches the spec's "no second config" posture and keeps the standing lane authoritative. Preserving the `0049` commands prevents regressing their closed survivors.
2. No backwards-compatibility shims: N/A (no code change); no second mutation config is introduced.

## Verification Layers

1. `INV-092`/`INV-098` (regression evidence is measured, not declared) → mutation campaign: focused `cargo mutants` over the changed functions reports caught/missed/unviable with exact denominators; the three `0049` commands rerun with zero new misses; the standing perimeter runs to completion.
2. Single-surface note: this is an evidence ticket; its one surface (mutation posture) maps to the recorded campaign transcripts — no source/replay layer applies because it changes nothing.

## What to Change

### 1. Run focused campaigns over the new branches

Run `cargo-mutants` focused over the functions changed by `-001`…`-009` (actor/process eligibility + cadence; declared-process invocation; closed actor-step outcome handling; `EventId` uniqueness; `matches_expected` temporal conjunct + typed first-divergence; salience predicates). Record caught/missed/unviable and the selected denominator.

### 2. Preserve and rerun the `0049` focused commands

Rerun the three commands recorded in `archive/tickets/0049MUTWIT-001..003.md` exactly; confirm zero new misses (the `0049` witnesses remain non-vacuous).

### 3. Run the configured standing perimeter

Run the standing `.cargo/mutants.toml` lane to completion; record the outcome. Surviving mutants are triaged (missing witness vs equivalent) per the repo's existing mutation-evidence process; remediation of any real survivor routes to a separate ticket.

## Files to Touch

- `None — evidence-only ticket; runs the configured + focused mutation campaigns and records denominators/artifacts. No `.cargo/mutants.toml` or source change (spec §6.3).`

## Out of Scope

- Any `.cargo/mutants.toml` perimeter change (spec §6.3: none).
- Fixing a real survivor — routes to a separate remediation ticket if the campaign surfaces one.
- The acceptance artifact assembly — `0050FOUCONSEC-013` (this ticket's recorded results feed it).

## Acceptance Criteria

### Tests That Must Pass

1. Focused campaigns over the new branches record exact caught/missed/unviable sets and denominators; any miss is triaged (missing witness vs equivalent) with a recorded resolution.
2. The three `0049` focused commands rerun with zero new misses; the standing perimeter runs to completion with its outcome recorded.
3. The campaign environment matches CI (flags/versions) so counts are trustworthy.

### Invariants

1. Mutation evidence is measured and recorded with exact denominators, not declared (`INV-092`/`INV-098`).
2. No second mutation config is introduced; the standing perimeter remains authoritative (spec §6.3).

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; verification is the recorded mutation transcripts. No tests change.`

### Commands

1. Focused: `cargo mutants -p tracewake-core` scoped to the changed functions (per `--file`/`--re` over the `-001`…`-009` surfaces).
2. Preserved `0049`: the three commands as recorded in `archive/tickets/0049MUTWIT-001..003.md`.
3. Standing: the configured `.cargo/mutants.toml` lane run to completion (matching the CI mutation environment).
