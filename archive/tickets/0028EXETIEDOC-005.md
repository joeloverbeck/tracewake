# 0028EXETIEDOC-005: Single-owner derived-accounting proof across scheduler, pipeline, replay, and goldens

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — doctrine edits to `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` (single-owner derived-accounting proof) plus cross-reference in `05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` and a byte-stable-insufficient artifact rule in `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` + `10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`. No crate/code, no fixtures.
**Deps**: 0028EXETIEDOC-006 (the doc-`10` artifact rule references the general anti-vacuity / behavior-witness standard from D8 in `10`). **Execution-blocking precondition**: owner approval per spec 0028 §R-A.

## Problem

D6 (report E05). Architecture `04`/`05` (ratified by spec 0027) now name a single-owner derived-accounting seam: decision transactions consume live need pressures and produce proposals but may not mint need deltas or charge durations directly — derived consequences belong at the action-pipeline/ordinary-life boundary. Execution `06` already proves strong single-tick need accounting (every delta traces to an event; active effects charge at most once per actor/need/tick; durations are tick-audited) but names no single-**owner** seam across scheduler/action/projection/replay/golden artifacts, and no rule that byte-stable goldens are insufficient if the semantic single-charge ledger drifts (verified: 0 `single-owner`/`seam`/`byte-stable` matches in `06` at `64a8367`). This ticket adds a single-owner derived-accounting proof to `06`, a proposal/scheduler non-ownership cross-reference to `05`, and a byte-stable-insufficient artifact rule to `09`/`10`.

## Assumption Reassessment (2026-06-13)

1. Verified against the live tree (`64a8367`): `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` already proves single-tick / duration accounting (need delta → event; ≤1 charge per actor/need/tick; tick-audited durations) and no-human ordinary-life proof, but `grep -inE "single-owner|single owner|seam|byte-stable"` returned 0 — no named single-owner seam and no byte-stable-but-false golden rule. `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` carries proposal/validation/scheduler boundaries and no-direct-dispatch proof, the right home for the proposal/scheduler non-ownership cross-reference.
2. Verified against spec 0028 (`specs/0028_EXECUTION_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md`) §3 D6 and the source report `reports/execution-tier-alignment-research-report.md` §E05. Upstream contract: the architecture `04`/`05`/`09` single-charge accounting seam, ratified by `archive/specs/0027_ARCHITECTURE_TIER_DOCTRINE_ALIGNMENT_AMENDMENT.md` D6.
3. Shared boundary under audit: the derived-accounting ownership line — only the action-pipeline/ordinary-life boundary (`06`) may emit need/duration deltas; scheduler/routine/planner/projection/replay/golden-normalization (`05`, `09`, `10`) may consume but not independently charge. The `10` artifact rule instances the D8 general standard (ticket 006).
4. Constitutional invariants motivating this ticket, restated before trusting the narrative: `INV-018` (deterministic replay reconstructs significant outcomes from ordered events) and `INV-009` (meaningful state changes require events). D6 proves no tick/window is causally charged twice across the pipeline, protecting replay truth — not merely byte-stable goldens.
5. Deterministic-replay surface (enforcement is the action-pipeline/ordinary-life accounting code + the `10` standard in ticket 006, not this edit): the proof requires the only delta-emitting layer to be the action-pipeline/ordinary-life boundary; scheduler/routine/planner/projection/replay/golden code must not independently charge or normalize deltas; and a golden's byte stability is insufficient if the semantic single-charge ledger drifts, double-counts, or omits a duration terminal. This strengthens deterministic-replay discipline (INV-018) and eventful-change discipline (INV-009). No code path, no nondeterminism added — proof doctrine + a golden-acceptance artifact rule only.

## Architecture Check

1. The seam proof lives in `06` (ordinary-life needs/no-human proof) because `06` already owns single-tick need accounting — naming the single-**owner** seam extends the existing accounting proof rather than creating a parallel one. `05` gets a proposal/scheduler non-ownership cross-reference (it owns the pipeline boundaries); `09`/`10` get the golden-acceptance artifact rule (they own fixture/review acceptance). This mirrors the architecture predecessor's A04/A05/A09 distribution (`archive/tickets/0027ARCTIEDOC-005.md`).
2. No backwards-compatibility aliasing/shims: additive proof obligations. `06`'s existing single-tick proof is extended with the single-owner seam; no existing accounting gate is weakened or duplicated.

## Verification Layers

1. `INV-018`/`INV-009` single-owner derived accounting → invariants alignment check + codebase grep-proof: `06` names the action-pipeline/ordinary-life boundary as the sole delta-emitting owner; scheduler/routine/planner/projection/replay/golden may consume but not charge.
2. Proposal/scheduler non-ownership → codebase grep-proof: `05` cross-references that candidate generation may explain pressure but must not mint need deltas or charge time.
3. Byte-stable-insufficient golden rule → manual review against the `10` standard (ticket 006): `09`/`10` state that a golden's byte stability is insufficient if the semantic single-charge ledger drifts, double-counts, or omits a duration terminal.

## What to Change

### 1. `06` — single-owner derived-accounting proof (D6)

Add a subsection (final wording at enactment): the only layer permitted to emit need/duration deltas is the action-pipeline/ordinary-life boundary. Scheduler, routine planner, projection, replay, and golden-output normalization may *consume* derived accounting but must not independently charge or reconcile the same tick/window. Need deltas, duration ticks, work completion/failure, and passive windows are charged exactly once by the owning seam.

### 2. `05` — proposal/scheduler non-ownership cross-reference (D6)

Add a cross-reference: actor decision transactions consume live need pressures and may explain pressure, but candidate generation must not mint need deltas, supply proposal-side current-need values as authority, or let routine labels charge time.

### 3. `09` + `10` — byte-stable-insufficient artifact rule (D6)

Add an artifact rule (referencing the D8 standard in ticket 006): a golden's byte stability is insufficient if the semantic single-charge ledger drifts, double-counts, or omits a duration terminal — semantic single-charge evidence is required beyond stable output bytes.

## Files to Touch

- `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` (modify)
- `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` (modify — non-ownership cross-reference)
- `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` (modify — byte-stable-insufficient rule; shared with 0028EXETIEDOC-002, additive)
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modify — byte-stable-insufficient rule; shared hub with 0028EXETIEDOC-003/004/006; references the D8 standard in 006)

## Out of Scope

- **The general anti-vacuity / behavior-witness standard in `10`** — D8, sibling ticket 0028EXETIEDOC-006 (this ticket's `10` rule instances it).
- **Redesigning the economy, scheduler, or accounting mechanism** — D6 proves the architecture seam; it designs no new economy/scheduler (spec §3 D6).
- **Owner approval itself (spec §R-A)** — a human owner act; this ticket's execution precondition, not its deliverable.
- Crate/code, fixtures, foundation/architecture/reference edits.

## Acceptance Criteria

### Tests That Must Pass

1. **Single-owner seam landing grep** — `06` names the single delta-emitting owner: `grep -niE "single-owner|owning seam|emit need|charge.*once|may not.*charge" docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` resolves the seam proof.
2. **Byte-stable-insufficient landing grep** — `09`/`10` state the golden-acceptance rule: `grep -niE "byte.stable|byte stability|semantic.*single.charge|double-count|duration terminal" docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` resolves the artifact rule.
3. **Invariants alignment review** — D6 proves no tick/window is causally charged twice across scheduler/pipeline/projection/replay/golden (INV-018/009); byte-stable goldens are insufficient without semantic single-charge evidence.

### Invariants

1. Only the action-pipeline/ordinary-life boundary emits need/duration deltas; no other layer independently charges the same tick/window (INV-018/009).
2. Golden byte stability is insufficient if the semantic single-charge ledger drifts, double-counts, or omits a duration terminal.

## Test Plan

### New/Modified Tests

1. `None — documentation-only execution-doctrine ticket; verification is command-based (seam + byte-stable landing greps) plus an invariants-alignment manual review against the architecture 04/05/09 seam. No crate/code or fixture changes, so existing pipeline coverage is unaffected.`

### Commands

1. `grep -niE "single-owner|owning seam|may not.*charge|charge.*once" docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` — confirms the single-owner seam proof.
2. `grep -niE "byte.stable|semantic.*single.charge|double-count|duration terminal" docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` — confirms the byte-stable-insufficient rule.
3. `Documentation-only: the four-gate Rust pipeline (fmt/clippy/build/test) is unaffected; the verification boundary for an execution-doc edit is the greps above plus the invariants-alignment review against architecture 04/05/09.`

## Outcome

Completed: 2026-06-13

Owner approval precondition: satisfied by the user's active `$ticket-series`
goal to implement `tickets/0028EXETIEDOC*` against
`specs/0028_EXECUTION*`.

Changed:

- Added the single-owner derived-accounting seam to
  `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`.
- Added scheduler/proposal non-ownership wording to
  `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`.
- Added byte-stable-insufficient golden acceptance language to
  `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`
  and `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`.

Verification:

- `grep -niE "single-owner|owning seam|emit need|charge.*once|may not.*charge" docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md`
  resolved the single-owner seam proof.
- `grep -niE "mint need|charge durations|current-need values|routine labels charge" docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`
  resolved the scheduler/proposal non-ownership wording.
- `grep -niE "byte.stable|byte stability|semantic.*single.charge|double-count|duration terminal" docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`
  resolved the byte-stable-insufficient artifact rule.
- Invariants alignment review: only the action-pipeline/ordinary-life boundary
  emits need/duration deltas, and byte-stable goldens cannot certify drifted,
  double-counted, or missing duration-terminal semantics.

Deviations:

- None. The Rust gate pipeline was not run for this ticket because the accepted
  verification boundary is documentation grep plus invariants review.
